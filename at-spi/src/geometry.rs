use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Debug, Type, Serialize, Hash, PartialEq)]
pub enum Coord {
    Screen,
    Window,
}

#[derive(Debug, Type, Deserialize, Hash, PartialEq)]
pub enum Layer {
    Invalid,
    Background,
    Canvas,
    Widget,
    Mdi,
    Popup,
    Overlay,
    Window,
}
