use at_spi::interface::*;

#[async_std::main]
async fn main() {
    let root: Accessible = at_spi::Object::default().iface().await.unwrap();
    let gnome: Accessible = root
        .get_child_at_index(0)
        .await
        .unwrap()
        .iface()
        .await
        .unwrap();
    println!("{}", gnome.name().await.unwrap());
}
