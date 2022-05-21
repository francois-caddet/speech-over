use flagset::{flags, FlagSet};
use serde::Deserialize;

use zbus::zvariant::Type;

#[derive(Debug, Type, PartialEq)]
#[zvariant(signature = "au")]
pub struct StateSet(FlagSet<State>);

use std::fmt;

use serde::de::{Deserializer, SeqAccess, Visitor};

struct StateVisitor;

impl<'de> Visitor<'de> for StateVisitor {
    type Value = StateSet;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a `Vec<u32>` of size 2.")
    }

    fn visit_seq<A>(self, mut value: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let s = value.next_element::<u32>()?.unwrap() as u64;
        let s = s | (value.next_element::<u32>()?.unwrap() as u64) << 32;
        Ok(StateSet(FlagSet::new(s).unwrap()))
    }
}

impl<'de> Deserialize<'de> for StateSet {
    fn deserialize<D>(deserializer: D) -> Result<StateSet, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(StateVisitor)
    }
}

flags! {
pub enum State : u64 {
    Invalid,
    Active,
    Armed,
    Busy,
    Checked,
    Collapsed,
    Defunct,
    Editable,
    Enabled,
    Expandable,
    Expanded,
    Focusable,
    Focused,
    HasTooltip,
    Horizontal,
    Iconified,
    Modal,
    MultiLine,
    Multiselectable,
    Opaque,
    Pressed,
    Resizable,
    Selectable,
    Selected,
    Sensitive,
    Showing,
    SingleLine,
    Stale,
    Transient,
    Vertical,
    Visible,
    ManagesDescendants,
    Indeterminate,
    Required,
    Truncated,
    Animated,
    InvalidEntry,
    SupportsAutocompletion,
    SelectableText,
    IsDefault,
    Visited,
    Checkable,
    HasPopup,
    ReadOnly,
}
}
