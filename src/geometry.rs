use euclid::{Point2D, Rect as Rectangle, Size2D};

pub struct ScreenSpace;
pub type Size = Size2D<i32, ScreenSpace>;
pub type Rect = Rectangle<i32, ScreenSpace>;
pub type Point = Point2D<i32, ScreenSpace>;
