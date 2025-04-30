use std::rc::Rc;

use crate::geometry::{Point, Rect, Size};

use super::{
    element::{ComposedElement, PlacedElement},
    layout::{Layout, PlacedLayout},
    widget::PlacedWidget,
};

pub struct ElementTree<S> {
    root: Rc<dyn Layout<Surface = S>>,
    placed_root: Option<PlacedLayout<S>>,
}

impl<S> ElementTree<S> {
    fn composite(&mut self, size: Size) {
        let rect = Rect::new(Point::new(0, 0), size);
        self.placed_root = Some(PlacedLayout {
            rect,
            children: self
                .root
                .composite(size)
                .iter()
                .map(|element| Self::place_element(element, rect.origin))
                .collect(),
        })
    }

    fn place_element(element: &ComposedElement<S>, origin: Point) -> PlacedElement<S> {
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
