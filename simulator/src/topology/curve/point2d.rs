use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Point2D {
    x: f32,
    y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Point2D { x, y }
    }

    pub fn distance(self, other: Self) -> f32 {
        let Point2D { x, y } = other - self;
        (x.powi(2) + y.powi(2)).sqrt()
    }
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output::new(self.x + other.x, self.y + other.y)
    }
}

impl Mul<f32> for Point2D {
    type Output = Self;

    fn mul(self, amount: f32) -> Self {
        Self::Output::new(self.x * amount, self.y * amount)
    }
}