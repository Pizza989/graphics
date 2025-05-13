use indexmap::IndexMap;
use std::rc::Rc;
use uuid::Uuid;

use crate::geometry::{Point, Rect, Size};

use super::{layout::Layout, ui_node::UiNode};

pub struct UiTree<T> {
    pub root: Rc<dyn Layout<Texture = T>>,
    pub layout_cache: Option<IndexMap<Uuid, Rect>>,
}

impl<T> UiTree<T> {
    pub fn new(root: Rc<dyn Layout<Texture = T>>) -> Self {
        Self {
            root,
            layout_cache: None,
        }
    }

    fn append_to_layout_cache(
        layout_cache: &mut IndexMap<Uuid, Rect>,
        indexmap: IndexMap<Uuid, Rect>,
    ) {
        for (k, v) in indexmap {
            layout_cache.insert(k, v);
        }
    }

    // TODO: figure out recursion
    pub fn layout_cache(&mut self, size: Size) -> IndexMap<Uuid, Rect> {
        let rect = Rect::new(Point::new(0, 0), size);
        let mut layout_cache = IndexMap::new();

        layout_cache.insert(self.root.id(), rect);
        let sub_cache = self.root.composite(size);
        for (id, rect) in &sub_cache {}
        Self::append_to_layout_cache(&mut layout_cache, sub_cache);
        layout_cache
    }

    pub fn render(&mut self, size: Size) {
        let layout_cache = self.layout_cache(size);

        for (k, v) in layout_cache {}
    }
}
