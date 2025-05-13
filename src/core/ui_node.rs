use super::layout::Layout;
use super::widget::Widget;
use std::rc::Rc;

pub enum UiNode<T> {
    Widget(Rc<dyn Widget<Texture = T>>),
    Layout(Rc<dyn Layout<Texture = T>>),
}
