use std::{any::Any, rc::Rc};

use crate::geometry::Rect;

pub trait Widget {
    type Texture;

    fn geometry(&self) -> &dyn Any;
    fn texture(&self) -> &Self::Texture;
    fn render(&self, size: (i32, i32));
}

pub struct PlacedWidget<T> {
    pub(crate) widget: Rc<dyn Widget<Texture = T>>,
    pub(crate) rect: Rect,
}
