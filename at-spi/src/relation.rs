use crate::Object;
use serde::Deserialize;
use zbus::zvariant::Type;

#[derive(Debug, Type, Deserialize, Hash, PartialEq)]
pub enum Relation {
    Null(Vec<Object>),
    LabelFor(Vec<Object>),
    LabelledBy(Vec<Object>),
    ControllerFor(Vec<Object>),
    ControlledBy(Vec<Object>),
    MemberOf(Vec<Object>),
    TooltipFor(Vec<Object>),
    NodeChildOf(Vec<Object>),
    NodeParentOf(Vec<Object>),
    Extended(Vec<Object>),
    FlowsTo(Vec<Object>),
    FlowsFrom(Vec<Object>),
    SubwindowOf(Vec<Object>),
    Embeds(Vec<Object>),
    EmbeddedBy(Vec<Object>),
    PopupFor(Vec<Object>),
    ParentWindowOf(Vec<Object>),
    DescriptionFor(Vec<Object>),
    DescribedBy(Vec<Object>),
    Details(Vec<Object>),
    DetailsFor(Vec<Object>),
    ErrorMessage(Vec<Object>),
    ErrorFor(Vec<Object>),
}
