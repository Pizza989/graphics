use crate::geometry::Rect;

use super::layout::{AbsoluteLayout, Layout};
use super::widget::{AbsoluteWidget, Widget};
use std::rc::Rc;

pub enum UiNode<T> {
    Widget(Rc<dyn Widget<Texture = T>>),
    Layout(Rc<dyn Layout<Texture = T>>),
}

pub enum LocallyAbsoluteUiNode<T> {
    Widget(AbsoluteWidget<T>),
    Layout {
        layout: Rc<dyn Layout<Texture = T>>,
        rect: Rect,
    },
}

pub enum AbsoluteUiNode<T> {
    Widget(AbsoluteWidget<T>),
    Layout(AbsoluteLayout<T>),
}
