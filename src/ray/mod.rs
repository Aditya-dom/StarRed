use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod test;