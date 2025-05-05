use std::rc::Rc;

use crate::geometry::{Point, Rect, Size};

use super::{
    element::{ComposedElement, PlacedElement},
    layout::{Layout, PlacedLayout},
    widget::PlacedWidget,
};

pub struct ElementTree<T> {
    pub root: Rc<dyn Layout<Surface = T>>,
    pub placed_tree: Option<PlacedElementTree<T>>,
}

pub struct PlacedElementTree<T> {
    pub root: PlacedLayout<T>,
}

impl<T> PlacedElementTree<T> {
    pub fn new(root: PlacedLayout<T>) -> Self {
        Self { root }
    }
}

impl<T> ElementTree<T> {
    pub fn composite(&mut self, size: Size) {
        let rect = Rect::new(Point::new(0, 0), size);

        let placed_layout = PlacedLayout {
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
            None => self.placed_tree = Some(PlacedElementTree::new(placed_layout)),
        }
    }

    fn place_element(element: &ComposedElement<T>, origin: Point) -> PlacedElement<T> {
        match element {
            ComposedElement::Widget(relatively_placed_widget) => {
                PlacedElement::Widget(PlacedWidget {
                    widget: relatively_placed_widget.widget.clone(),
                    rect: relatively_placed_widget.rect.translate(origin.to_vector()),
                })
            }
            ComposedElement::Layout { layout, rect } => {
                let placed_children = layout
                    .composite(rect.size)
                    .iter()
                    .map(|element| Self::place_element(element, rect.origin))
                    .collect();

                PlacedElement::Layout(PlacedLayout {
                    rect: rect.translate(origin.to_vector()),
                    children: placed_children,
                })
            }
        }
    }
}
