use std::{any::Any, rc::Rc};
use uuid::Uuid;

/// Widgets are renderable Entities that can be referenced by id
pub trait Widget {
    type Texture;

    fn geometry(&self) -> &dyn Any;
    fn texture(&self) -> &Self::Texture;
    fn render(&self, size: (i32, i32));
}
