use at_spi::interface::*;

#[async_std::main]
async fn main() {
    let root: Accessible = at_spi::Object::default().iface().await.unwrap();
    for w in root.get_children().await.unwrap() {
        let app: Accessible = w.iface().await.unwrap();
        for a in app.get_children().await.unwrap() {
            let gnomew: Accessible = a.iface().await.unwrap();
            println!("{:#?}", gnomew.name().await);
            println!("{:#?}", gnomew.get_interfaces().await);
            println!("{:#?}", gnomew.get_relation_set().await);
            println!("{:#?}", gnomew.get_role().await);
            println!("{:#?}", gnomew.get_state().await);
        }
    }
}
