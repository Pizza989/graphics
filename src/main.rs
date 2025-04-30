use std::{any::Any, rc::Rc};

struct Rectangle {
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
}

trait Widget {
    type Geometry;
    type Surface;

    fn geometry(&self) -> &Self::Geometry;
    fn surface(&self) -> &Self::Surface;
    fn render(&self, size: (i32, i32));
}

trait Layout {
    type Surface;

    fn children(&self) -> Vec<Element<Self::Surface>>;
    fn composite(&self, size: (u32, u32)) -> [PlacedElement<Self::Surface>];
}

struct PlacedWidget<G: ?Sized, S> {
    widget: Rc<dyn Widget<Geometry = G, Surface = S>>,
    rect: Rectangle,
}

struct PlacedLayout<S> {
    layout: Rc<dyn Layout<Surface = S>>,
    rect: Rectangle,
}

enum PlacedElement<S> {
    Widget(PlacedWidget<dyn Any, S>),
    Layout(PlacedLayout<S>),
}

enum Element<S> {
    Widget(Rc<dyn Widget<Geometry = dyn Any, Surface = S>>),
    Layout(Rc<dyn Layout<Surface = S>>),
}

struct ElementTree<S> {
    root: Rc<dyn Layout<Surface = S>>,
    placed_root: Option<PlacedLayout<S>>,
}

impl<S> ElementTree<S> {
    fn composite(&mut self, size: (i32, i32)) {
        self.placed_root = Some(PlacedLayout {
            layout: self.root.clone(),
            rect: Rectangle {
                top: 0,
                bottom: size.1,
                left: 0,
                right: size.0,
            },
        });
    }
}

trait UserInterface<'a, S> {
    fn tree() -> &'a ElementTree<'a, S>;
    fn new() -> Self;
    fn dispatch_event();
}

fn main() {
    println!("Hello, world!");
}
