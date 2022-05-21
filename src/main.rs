use at_spi::interface::*;

#[async_std::main]
async fn main() {
    let root: Accessible = at_spi::Object::default().iface().await.unwrap();
    for w in root.get_children().await.unwrap() {
        let app: Accessible = w.iface().await.unwrap();
        for a in app.get_children().await.unwrap() {
            //            let win: Accessible = a.iface().await.unwrap();
            //            for w in win.get_children().await.unwrap() {
            let access: Accessible = a.iface().await.unwrap();
            let comp: Component = a.iface().await.unwrap();
            comp.set_position(700, 700, at_spi::geometry::Coord::Window)
                .await
                .unwrap();
            comp.set_size(700, 700).await.unwrap();

            println!("{:#?}", access.name().await);
            println!("{:#?}", access.get_interfaces().await);
            println!("{:#?}", access.get_relation_set().await);
            println!("{:#?}", access.get_role().await);
            println!("{:#?}", access.get_state().await);

            println!(
                "{:?}",
                comp.get_extents(at_spi::geometry::Coord::Screen).await
            );
            println!("{:?}", comp.get_mdiz_order().await);
            println!("{:?}", comp.get_layer().await);
            //            }
        }
    }
}
