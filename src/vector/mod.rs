use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;
        let z2 = self.z * self.z;

        (x2 + y2 + z2).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn lerp(&self, other: Self, t: f64) -> Self {
        *self * (1.0 - t) + other * t
    }

    pub fn dot(&self, other: Self) -> f64 {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;

        x + y + z
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let x = (self.y * other.z) - (self.z * other.y);
        let y = (self.z * other.x) - (self.x * other.z);
        let z = (self.x * other.y) - (self.y * other.x);

        Self::new(x, y, z)
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, divisor: f64) -> Self::Output {
        self * (1.0 / divisor)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod test;