use crate::primitive::Primitive;
use crate::light::Light;
use crate::ray::Ray;
use crate::interaction::Interaction;

pub struct Scene {
    pub primitives: Vec<Primitive>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new(primitives: Vec<Primitive>, lights: Vec<Light>) -> Self {
        Self { primitives, lights }
    }

    pub fn intersection(&self, ray: Ray) -> Option<(Interaction, &Primitive)> {
        self.primitives.iter()
            .filter_map(|p| p.intersection(ray).map(|i| (i, p)))
            .min_by_key(|&(i, _)| i)
    }
}

#[cfg(test)]
mod test;