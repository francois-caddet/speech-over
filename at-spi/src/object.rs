use serde::Deserialize;

use zbus::zvariant::{OwnedObjectPath, Type};
use zbus::ProxyBuilder;

#[derive(Debug, Type, Deserialize, Hash, PartialEq)]
pub struct Object {
    name: String,
    path: OwnedObjectPath,
}

impl Object {
    pub async fn iface<'i, I>(self) -> zbus::Result<I>
    where
        I: From<zbus::Proxy<'i>> + zbus::ProxyDefault,
    {
        ProxyBuilder::new(crate::CONNECTION.await)
            .destination(self.name)?
            .path(self.path)?
            .interface(I::INTERFACE)?
            .build()
            .await
    }
}
