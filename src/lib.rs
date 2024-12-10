pub mod template;

pub type Point<T = i32> = (T, T);

pub struct Vec2<T = i32> {
    pub x: T,
    pub y: T,
}

impl<T> From<Point<T>> for Vec2<T> {
    fn from(tuple: Point<T>) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }
}

pub fn out_of_bounds<T>(y: i32, x: i32, matrix: &[Vec<T>]) -> bool {
    y < 0 || x < 0 || y > matrix.len() as i32 - 1 || x > matrix[0].len() as i32 - 1
}

// Use this file to add helper functions and additional modules.
