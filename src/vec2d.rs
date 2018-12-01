use std::ops::Add;

#[macro_export]
macro_rules! v2 {
    ( $x:expr, $y:expr ) => {
        Vec2d::new($x, $y)
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2d {
    pub x: i32,
    pub y: i32,
}

impl Vec2d {
    pub fn new(x: i32, y: i32) -> Vec2d {
        Vec2d { x, y }
    }

    pub fn directions() -> Vec<Vec2d> {
        vec![v2!(1, 0), v2!(0, 1), v2!(-1, 0), v2!(0, -1)]
    }
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Vec2d {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vec2d { x, y }
    }
}

impl<'a> Add<&'a Vec2d> for Vec2d {
    type Output = Vec2d;

    fn add(self, other: &'a Vec2d) -> Vec2d {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vec2d { x, y }
    }
}

impl<'a> Add<Vec2d> for &'a Vec2d {
    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Vec2d {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vec2d { x, y }
    }
}

impl<'a> Add<&'a Vec2d> for &'a Vec2d {
    type Output = Vec2d;

    fn add(self, other: &'a Vec2d) -> Vec2d {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vec2d { x, y }
    }
}
