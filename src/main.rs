use std::any::Any;

struct Rectangle {
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
}

struct PlacedWidget<'a, Geometry: ?Sized, Surface> {
    widget: &'a dyn Widget<Geometry>,
    rect: Rectangle,
}

struct PlacedLayout<'a> {
    layout: &'a dyn Layout,
    rect: Rectangle,
}

trait Widget<Geometry> {
    type Surface;

    fn geometry(&self) -> Geometry;
    fn surface(&self) -> &Self::Surface;
    fn render(&self, size: (i32, i32));
}

trait Layout {
    type Surface;

    fn children(&self) -> Vec<Element>;
    fn composite(&self, size: (u32, u32)) -> [PlacedElement<Self::Surface>];
}

enum PlacedElement<'a, Surface> {
    Widget(PlacedWidget<'a, dyn Any, Surface>),
    Layout(PlacedLayout<'a>),
}

enum Element {
    Widget(Box<dyn Widget>),
    Layout(Box<dyn Layout>),
}

struct ElementTree {
    root: Element,
}

trait UserInterface<'tree> {
    fn tree() -> &'tree ElementTree;
    fn new() -> Self;
    fn dispatch_event();
}

fn main() {
    println!("Hello, world!");
}
