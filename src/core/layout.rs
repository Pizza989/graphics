use super::ui_node::{AbsoluteUiNode, LocallyAbsoluteUiNode, UiNode};
use crate::geometry::{Rect, Size};

pub trait Layout {
    type Texture;

    fn children(&self) -> &Vec<UiNode<Self::Texture>>;
    fn composite(&self, size: Size) -> Vec<LocallyAbsoluteUiNode<Self::Texture>>;
}

pub struct AbsoluteLayout<T> {
    pub rect: Rect,
    pub(crate) children: Vec<AbsoluteUiNode<T>>,
}
