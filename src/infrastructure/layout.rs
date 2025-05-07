use super::element::{ComposedElement, Element, PlacedElement};
use crate::geometry::{Rect, Size};

pub trait Layout {
    type Texture;

    fn children(&self) -> &Vec<Element<Self::Texture>>;
    fn composite(&self, size: Size) -> Vec<ComposedElement<Self::Texture>>;
}

pub struct PlacedLayout<T> {
    pub rect: Rect,
    pub(crate) children: Vec<PlacedElement<T>>,
}
