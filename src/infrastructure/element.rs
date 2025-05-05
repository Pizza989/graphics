use crate::geometry::Rect;

use super::layout::{Layout, PlacedLayout};
use super::widget::{PlacedWidget, Widget};
use std::any::Any;
use std::rc::Rc;

pub enum Element<T> {
    Widget(Rc<dyn Widget<Geometry = dyn Any, Texture = T>>),
    Layout(Rc<dyn Layout<Surface = T>>),
}

pub enum ComposedElement<T> {
    Widget(PlacedWidget<dyn Any, T>),
    Layout {
        layout: Rc<dyn Layout<Surface = T>>,
        rect: Rect,
    },
}

pub enum PlacedElement<T> {
    Widget(PlacedWidget<dyn Any, T>),
    Layout(PlacedLayout<T>),
}
