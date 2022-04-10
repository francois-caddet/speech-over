{
  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  inputs.pre-commit-hooks.inputs.nixpkgs.follows = "nixpkgs";
  inputs.pre-commit-hooks.inputs.flake-utils.follows = "flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  inputs.rust-overlay.inputs.flake-utils.follows = "flake-utils";
  inputs.import-cargo.url = "github:edolstra/import-cargo";

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , pre-commit-hooks
    , rust-overlay
    , import-cargo
    }:
    let
      SYSTEMS = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      RUST_CHANNELS = [
        "stable"
        "beta"
        "nightly"
      ];

      forEachRustChannel = fn: builtins.listToAttrs (builtins.map fn RUST_CHANNELS);

      cargoTOML = builtins.fromTOML (builtins.readFile ./Cargo.toml);

      version = "${cargoTOML.package.version}_${builtins.substring 0 8 self.lastModifiedDate}_${self.shortRev or "dirty"}";

    in
    flake-utils.lib.eachSystem SYSTEMS (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
        ];
      };

      cargoHome = (import-cargo.builders.importCargo {
        lockFile = ./Cargo.lock;
        inherit pkgs;
      }).cargoHome;

      mkRust =
        { rustProfile ? "minimal"
        , rustExtensions ? [
            "rust-src"
            "rust-analysis"
            "rustfmt-preview"
            "clippy-preview"
          ]
        , channel ? "stable"
        , target ? pkgs.rust.toRustTarget pkgs.stdenv.hostPlatform
        }:
        let
          _rust =
            if channel == "nightly" then
              pkgs.rust-bin.selectLatestNightlyWith
                (toolchain: toolchain.${rustProfile}.override {
                  extensions = rustExtensions;
                  targets = [ target ];
                })
            else
              pkgs.rust-bin.${channel}.latest.${rustProfile}.override {
                extensions = rustExtensions;
                targets = [ target ];
              };
        in
        pkgs.buildEnv {
          name = _rust.name;
          inherit (_rust) meta;
          buildInputs = [ pkgs.makeWrapper ];
          paths = [ _rust ];
          pathsToLink = [ "/" "/bin" ];
          # XXX: This is needed because cargo and clippy commands need to
          # also be aware of other binaries in order to work properly.
          # https://github.com/cachix/pre-commit-hooks.nix/issues/126
          postBuild = ''
            for i in $out/bin/*; do
              wrapProgram "$i" --prefix PATH : "$out/bin"
            done

          '';
        };

      buildSpeechOver =
        { channel ? "stable"
        , isDevShell ? false
        , target ? pkgs.rust.toRustTarget pkgs.stdenv.hostPlatform
        ,
        }:
        let
          rustProfile =
            if isDevShell then "default"
            else "minimal";

          rust = mkRust { inherit rustProfile channel target; };

          pre-commit = pre-commit-hooks.lib.${system}.run {
            src = self;
            hooks = {
              nixpkgs-fmt = {
                enable = true;
              };
              rustfmt = {
                enable = true;
                entry = pkgs.lib.mkForce "${rust}/bin/cargo-fmt fmt -- --check --color always";
              };
            };
          };

        in
        pkgs.stdenv.mkDerivation {
          name = "speech-over-${version}";

          buildInputs =
            [
              rust
            ] ++ (if isDevShell then [ ]
            else [ cargoHome ]);

          src = if isDevShell then null else self;

          buildPhase = ''
            cargo build --workspace --release --frozen --offline
          '';

          doCheck = true;

          checkPhase = ''
            cargo test --release --frozen --offline
          '' + (pkgs.lib.optionalString (channel == "stable") ''
            cargo fmt --all -- --check
          '');

          installPhase = ''
            mkdir -p $out
            cargo install --frozen --offline --path . --root $out
            rm $out/.crates.toml
          '';

          shellHook = pre-commit.shellHook + ''
            echo "=== SpeechOver development shell ==="
          '';

          passthru = { inherit rust pre-commit; };

          RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
        };
    in
    rec {
      defaultPackage = packages.build;
      packages = {
        build = buildSpeechOver { };
      };

      devShell = devShells.stable;
      devShells = forEachRustChannel
        (channel: {
          name = channel;
          value = buildSpeechOver { inherit channel; isDevShell = true; };
        });

      checks = {
        pre-commit = defaultPackage.pre-commit;
      } // (forEachRustChannel (channel:
        {
          name = "speech-over-against-${channel}-rust-channel";
          value = buildSpeechOver { inherit channel; };
        }
      ));
    }
    );
}

