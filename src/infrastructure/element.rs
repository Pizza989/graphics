use crate::geometry::Rect;

use super::layout::{Layout, PlacedLayout};
use super::widget::{PlacedWidget, Widget};
use std::any::Any;
use std::rc::Rc;

pub enum Element<T> {
    Widget(Rc<dyn Widget<Texture = T>>),
    Layout(Rc<dyn Layout<Texture = T>>),
}

pub enum ComposedElement<T> {
    Widget(PlacedWidget<T>),
    Layout {
        layout: Rc<dyn Layout<Texture = T>>,
        rect: Rect,
    },
}

pub enum PlacedElement<T> {
    Widget(PlacedWidget<T>),
    Layout(PlacedLayout<T>),
}
