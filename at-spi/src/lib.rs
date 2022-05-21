pub mod interface;
mod object;
pub use object::Object;
mod bus;
pub use bus::{BusProxy as Bus, StatusProxy as Status};
mod relation;
pub use relation::Relation;
mod role;
pub use role::Role;
mod state;
pub use state::State;
pub use state::StateSet;
pub mod geometry;

use async_static::async_static;

async_static! {
    pub static ref CONNECTION: zbus::Connection = connect().await.unwrap();
}

async fn connect() -> zbus::Result<zbus::Connection> {
    let session = zbus::Connection::session().await?;
    let bus = Bus::new(&session).await?;
    let name = bus.get_address().await?;
    zbus::ConnectionBuilder::address(name.as_str())?
        .build()
        .await
}
