use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Vector,
    pub reflectance: f64,
}

impl Material {
    pub fn new(color: Vector) -> Self {
        Self { color, reflectance: 0.0 }
    }
}

#[cfg(test)]
mod test;