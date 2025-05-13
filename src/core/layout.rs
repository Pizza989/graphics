use crate::core::ui_node::UiNode;
use crate::geometry::{Rect, Size};
use indexmap::IndexMap;
use uuid::Uuid;

pub trait Layout {
    type Texture;

    fn children(&self) -> &Vec<UiNode<Self::Texture>>;
    fn composite(&self, size: Size) -> IndexMap<Uuid, Rect>;
    fn id(&self) -> Uuid;
}
