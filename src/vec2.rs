use std::ops::Add;

#[macro_export]
macro_rules! v2 {
    ( $x:expr, $y:expr ) => {
        Vec2::new($x, $y)
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn directions() -> Vec<Vec2> {
        vec![v2!(1, 0), v2!(0, 1), v2!(-1, 0), v2!(0, -1)]
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vec2 { x, y }
    }
}

impl<'a> Add<&'a Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: &'a Vec2) -> Vec2 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vec2 { x, y }
    }
}

impl<'a> Add<Vec2> for &'a Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vec2 { x, y }
    }
}

impl<'a> Add<&'a Vec2> for &'a Vec2 {
    type Output = Vec2;

    fn add(self, other: &'a Vec2) -> Vec2 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vec2 { x, y }
    }
}
