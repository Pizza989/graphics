use std::rc::Rc;

use crate::geometry::{Point, Rect, Size};

use super::{
    layout::{AbsoluteLayout, Layout},
    ui_node::{AbsoluteUiNode, LocallyAbsoluteUiNode},
    widget::AbsoluteWidget,
};

pub struct UiTree<T> {
    pub root: Rc<dyn Layout<Texture = T>>,
    pub placed_tree: Option<AbsoluteUiTree<T>>,
}

pub struct AbsoluteUiTree<T> {
    pub root: AbsoluteLayout<T>,
}

impl<T> AbsoluteUiTree<T> {
    pub fn new(root: AbsoluteLayout<T>) -> Self {
        Self { root }
    }
}

impl<T> UiTree<T> {
    pub fn new(root: Rc<dyn Layout<Texture = T>>) -> Self {
        Self {
            root,
            placed_tree: None,
        }
    }
    pub fn composite(&mut self, size: Size) {
        let rect = Rect::new(Point::new(0, 0), size);

        let placed_layout = AbsoluteLayout {
            rect,
            children: self
                .root
                .composite(size)
                .iter()
                .map(|element| Self::place_element(element, rect.origin))
                .collect(),
        };

        match &mut self.placed_tree {
            Some(tree) => tree.root = placed_layout,
            None => self.placed_tree = Some(AbsoluteUiTree::new(placed_layout)),
        }
    }

    fn place_element(element: &LocallyAbsoluteUiNode<T>, origin: Point) -> AbsoluteUiNode<T> {
        match element {
            LocallyAbsoluteUiNode::Widget(relatively_placed_widget) => {
                AbsoluteUiNode::Widget(AbsoluteWidget {
                    widget: relatively_placed_widget.widget.clone(),
                    rect: relatively_placed_widget.rect.translate(origin.to_vector()),
                })
            }
            LocallyAbsoluteUiNode::Layout { layout, rect } => {
                let placed_children = layout
                    .composite(rect.size)
                    .iter()
                    .map(|element| Self::place_element(element, rect.origin))
                    .collect();

                AbsoluteUiNode::Layout(AbsoluteLayout {
                    rect: rect.translate(origin.to_vector()),
                    children: placed_children,
                })
            }
        }
    }
}
