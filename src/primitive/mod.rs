use crate::sphere::Sphere;
use crate::material::Material;
use crate::interaction::Interaction;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Primitive {
    pub shape: Sphere, // TODO: add a Shape trait
    pub material: Material,
}

impl Primitive {
    pub fn new(shape: Sphere, material: Material) -> Self {
        Self { shape, material }
    }

    pub fn intersection(&self, ray: Ray) -> Option<Interaction> {
        self.shape.intersection(ray)
    }
}

#[cfg(test)]
mod test;