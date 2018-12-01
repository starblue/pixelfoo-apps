use rand::Rng;

use crate::p2;
use crate::point2d::Point2d;
use crate::v2;
use crate::vec2d::Vec2d;

pub struct Rect2d {
    origin: Point2d,
    size: Vec2d,
}

impl Rect2d {
    pub fn new(x0: i32, x1: i32, y0: i32, y1: i32) -> Rect2d {
        assert!(x0 <= x1 && y0 <= y1);
        let origin = p2!(x0, y0);
        let size = v2!(x1 - x0, y1 - y0);
        Rect2d { origin, size }
    }

    pub fn left(&self) -> i32 {
        self.origin.x
    }
    pub fn right(&self) -> i32 {
        self.origin.x + self.size.x
    }

    pub fn bottom(&self) -> i32 {
        self.origin.y
    }
    pub fn top(&self) -> i32 {
        self.origin.y + self.size.y
    }

    pub fn width(&self) -> i32 {
        self.size.x
    }
    pub fn height(&self) -> i32 {
        self.size.y
    }

    pub fn contains(&self, p: Point2d) -> bool {
        self.left() <= p.x && p.x < self.right() && self.bottom() <= p.y && p.y < self.top()
    }

    pub fn random_point<R>(&self, rng: &mut R) -> Point2d
    where
        R: Rng,
    {
        let x = rng.gen_range(self.left(), self.right());
        let y = rng.gen_range(self.bottom(), self.top());
        p2!(x, y)
    }
}
