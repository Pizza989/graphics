use super::layout::Layout;
use super::widget::Widget;
use std::cell::{Ref, RefCell};
use std::clone::Clone;
use std::rc::Rc;
use uuid::Uuid;

pub type WidgetRefMut<T> = Rc<RefCell<dyn Widget<Texture = T>>>;
pub type WidgetRef<'a, T> = Ref<'a, dyn Widget<Texture = T>>;
pub type LayoutRefMut<T> = Rc<RefCell<dyn Layout<Texture = T>>>;
pub type LayoutRef<'a, T> = Ref<'a, dyn Layout<Texture = T>>;
pub type NodeId = Uuid;

pub enum UiElement<T> {
    Widget(Box<dyn Widget<Texture = T>>),
    Layout(Box<dyn Layout<Texture = T>>),
}

pub enum UiNode<'a, T: Clone> {
    Widget { widget_ref: WidgetRef<'a, T> },
    Layout { layout_ref: LayoutRef<'a, T> },
}

impl<'a, T: Clone> From<&'a UiNodeMut<T>> for UiNode<'a, T> {
    fn from(node_mut: &'a UiNodeMut<T>) -> Self {
        match node_mut {
            UiNodeMut::Widget { widget_ref } => UiNode::Widget {
                widget_ref: widget_ref.borrow(),
            },
            UiNodeMut::Layout { layout_ref } => UiNode::Layout {
                layout_ref: layout_ref.borrow(),
            },
        }
    }
}

/// Nodes don't store an id. This information is only kept in the Hashmap
/// on the UiTree, that creates a Relation between NodeId and UiNodeMut
#[derive(Clone)]
pub enum UiNodeMut<T: Clone> {
    Widget { widget_ref: WidgetRefMut<T> },
    Layout { layout_ref: LayoutRefMut<T> },
}

impl<T: Clone> From<WidgetRefMut<T>> for UiNodeMut<T> {
    fn from(widget_ref: WidgetRefMut<T>) -> Self {
        UiNodeMut::Widget { widget_ref }
    }
}

impl<T: Clone> From<LayoutRefMut<T>> for UiNodeMut<T> {
    fn from(layout_ref: LayoutRefMut<T>) -> Self {
        UiNodeMut::Layout { layout_ref }
    }
}

// TODO: figure this out, maybe look into use Cell instead of RefCell
// but maybe the solution is much easier aswell.
impl<T: Clone> From<UiElement<T>> for UiNodeMut<T> {
    fn from(ui_element: UiElement<T>) -> Self {
        match ui_element {
            UiElement::Widget(widget) => UiNodeMut::Widget {
                widget_ref: Rc::new(RefCell::from(widget)),
            },
            UiElement::Layout(layout) => todo!(),
        }
    }
}
