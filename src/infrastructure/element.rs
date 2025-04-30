use crate::geometry::Rect;

use super::layout::{Layout, PlacedLayout};
use super::widget::{PlacedWidget, Widget};
use std::any::Any;
use std::rc::Rc;

pub enum Element<S> {
    Widget(Rc<dyn Widget<Geometry = dyn Any, Surface = S>>),
    Layout(Rc<dyn Layout<Surface = S>>),
}

pub enum ComposedElement<S> {
    Widget(PlacedWidget<dyn Any, S>),
    Layout {
        layout: Rc<dyn Layout<Surface = S>>,
        rect: Rect,
    },
}

pub enum PlacedElement<S> {
    Widget(PlacedWidget<dyn Any, S>),
    Layout(PlacedLayout<S>),
}
