use std::ops::Add;

use crate::vec2::Vec2;

#[macro_export]
macro_rules! p2 {
    ( $x:expr, $y:expr ) => {
        Point2::new($x, $y)
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

impl Point2 {
    pub fn new(x: i32, y: i32) -> Point2 {
        Point2 { x, y }
    }

    pub fn neighbours(&self) -> Vec<Point2> {
        Vec2::directions().iter().map(|v| self + v).collect()
    }
}

impl Add<Vec2> for Point2 {
    type Output = Point2;

    fn add(self, other: Vec2) -> Point2 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Point2 { x, y }
    }
}

impl<'a> Add<&'a Vec2> for Point2 {
    type Output = Point2;

    fn add(self, other: &'a Vec2) -> Point2 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Point2 { x, y }
    }
}

impl<'a> Add<Vec2> for &'a Point2 {
    type Output = Point2;

    fn add(self, other: Vec2) -> Point2 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Point2 { x, y }
    }
}

impl<'a> Add<&'a Vec2> for &'a Point2 {
    type Output = Point2;

    fn add(self, other: &'a Vec2) -> Point2 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Point2 { x, y }
    }
}
