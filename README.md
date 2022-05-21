# SpeechOver

A modern screen reader for linux writen in rust.
The main target is mobile linux. First on PinePhone. This because, I have a
PinePhone I want to be accessible by blind people. Also because I don't pretend to
replace Orca on desktop. This could become an Orca alternative in the futur, but
that's not in my priorities for now.

## Build

The recomanded way to build it is using nix:

```bash
nix build
```

Anyway it's a purely rust software so it can be build by:

```bash
cargo build
```

## Install

Do not make sens to install it now. It's more than early and does not eaven build sometimes.

## Contribute

I don't accept contribution at this moment. It's too early.
Anyway, if you want to hack it, you can perform `nix develop` and you will be in the standard dev environment.
