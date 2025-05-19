use crate::core::ui_node::UiNodeMut;
use crate::geometry::{Rect, Size};
use indexmap::IndexMap;
use uuid::Uuid;

use super::ui_node::NodeId;
use super::ui_tree::GetNodeError;

pub trait Layout {
    type Texture: std::clone::Clone;

    fn children(&self) -> &Vec<UiNodeMut<Self::Texture>>;
    fn composite(&self, size: Size) -> IndexMap<Uuid, Rect>;
    fn add_child(&mut self, node: UiNodeMut<Self::Texture>);
    fn replace_child(
        &mut self,
        id: NodeId,
        node: UiNodeMut<Self::Texture>,
    ) -> Result<(), GetNodeError>;
}
