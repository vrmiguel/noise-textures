use sdl2::rect::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
/// Simple three-dimensional euclidian vector
pub struct Vec2<T: Copy> {
    pub x: T,
    pub y: T,
}

impl From<Point> for Vec2<i32> {
    fn from(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl Into<(i32, i32)> for Vec2<i32> {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Into<Point> for Vec2<i32> {
    fn into(self) -> Point {
        // Is this necessary?
        let pair: (i32, i32) = self.into();
        pair.into()
    }
}
