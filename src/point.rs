use std::ops::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    // Treats the point like a vector to normalize it
    #[allow(dead_code)]
    pub fn normalize(&mut self) {
        let magnitude = (self.x*self.x + self.y*self.y).powf(0.5);
        self.x /= magnitude;
        self.y /= magnitude;
    }
    // Treats the point like a vector to get its length
    #[allow(dead_code)]
    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y).powf(0.5)
    }
}

// Caution: implementation hell ahead

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add<Self> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Self> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Self> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Self> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<Self> for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign<Self> for Point {
    fn mul_assign(&mut self, rhs: Point) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Div<Self> for Point {
    type Output = Point;

    fn div(self, rhs: Point) -> Self {
        Point {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign<Self> for Point {
    fn div_assign(&mut self, rhs: Point) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

// Implementations for f32

impl Add<f32> for Point {
    type Output = Point;

    fn add(self, rhs: f32) -> Self {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign<f32> for Point {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Sub<f32> for Point {
    type Output = Point;

    fn sub(self, rhs: f32) -> Self {
        Point {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign<f32> for Point {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Point {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}