use super::element::{ComposedElement, Element, PlacedElement};
use crate::geometry::{Rect, Size};

pub trait Layout {
    type Surface;

    fn children(&self) -> Vec<Element<Self::Surface>>;
    fn composite(&self, size: Size) -> Vec<ComposedElement<Self::Surface>>;
}

pub struct PlacedLayout<T> {
    pub(crate) rect: Rect,
    pub(crate) children: Vec<PlacedElement<T>>,
}
