use crate::vector::Vector;
use crate::interaction::Interaction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    pub origin: Vector,
    pub power: f64,
}

impl Light {
    pub fn new(origin: Vector, power: f64) -> Self {
        Self { origin, power }
    }

    pub fn sample_li(&self, interaction: &Interaction) -> (f64, Vector) {
        let direction = self.origin - interaction.origin;

        let radius = direction.length();
        let radiance = self.power / radius.powf(2.0);

        (radiance, direction.normalize())
    }
}

#[cfg(test)]
mod test;