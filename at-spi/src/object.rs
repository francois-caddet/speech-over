use serde::Deserialize;

use zbus::zvariant::{OwnedObjectPath, Type};
use zbus::ProxyBuilder;

#[derive(Debug, Type, Deserialize, Hash, PartialEq)]
pub struct Object {
    name: String,
    path: OwnedObjectPath,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            name: String::from("org.a11y.atspi.Registry"),
            path: OwnedObjectPath::try_from("/org/a11y/atspi/accessible/root").unwrap(),
        }
    }
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
