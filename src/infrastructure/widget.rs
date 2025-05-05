use std::rc::Rc;

use crate::geometry::Rect;

pub trait Widget {
    type Geometry;
    type Texture;

    fn geometry(&self) -> &Self::Geometry;
    fn texture(&self) -> &Self::Texture;
    fn render(&self, size: (i32, i32));
}

pub struct PlacedWidget<G: ?Sized, T> {
    pub(crate) widget: Rc<dyn Widget<Geometry = G, Texture = T>>,
    pub(crate) rect: Rect,
}
