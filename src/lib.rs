use crate::infrastructure::element_tree::ElementTree;

mod geometry;
mod infrastructure;

trait UserInterface<'a, S> {
    fn tree() -> &'a ElementTree<S>;
    fn new() -> Self;
    fn dispatch_event();
}

fn main() {
    println!("Hello, world!");
}
