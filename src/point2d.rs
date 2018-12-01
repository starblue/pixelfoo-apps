use std::ops::Add;

use crate::vec2d::Vec2d;

#[macro_export]
macro_rules! p2 {
    ( $x:expr, $y:expr ) => {
        Point2d::new($x, $y)
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
}

impl Point2d {
    pub fn new(x: i32, y: i32) -> Point2d {
        Point2d { x, y }
    }

    pub fn neighbours(&self) -> Vec<Point2d> {
        Vec2d::directions().iter().map(|v| self + v).collect()
    }
}

impl Add<Vec2d> for Point2d {
    type Output = Point2d;

    fn add(self, other: Vec2d) -> Point2d {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Point2d { x, y }
    }
}

impl<'a> Add<&'a Vec2d> for Point2d {
    type Output = Point2d;

    fn add(self, other: &'a Vec2d) -> Point2d {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Point2d { x, y }
    }
}

impl<'a> Add<Vec2d> for &'a Point2d {
    type Output = Point2d;

    fn add(self, other: Vec2d) -> Point2d {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Point2d { x, y }
    }
}

impl<'a> Add<&'a Vec2d> for &'a Point2d {
    type Output = Point2d;

    fn add(self, other: &'a Vec2d) -> Point2d {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Point2d { x, y }
    }
}
