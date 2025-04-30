use std::rc::Rc;

use crate::geometry::Rect;

pub trait Widget {
    type Geometry;
    type Surface;

    fn geometry(&self) -> &Self::Geometry;
    fn surface(&self) -> &Self::Surface;
    fn render(&self, size: (i32, i32));
}

pub struct PlacedWidget<G: ?Sized, S> {
    pub(crate) widget: Rc<dyn Widget<Geometry = G, Surface = S>>,
    pub(crate) rect: Rect,
}
