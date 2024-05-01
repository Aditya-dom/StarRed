use std::cmp::{PartialOrd, Ord, Ordering};
use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interaction {
    pub ray_t: f64,
    pub origin: Vector,
    pub normal: Vector,
}

impl Interaction {
    pub fn new(ray_t: f64, origin: Vector, normal: Vector) -> Self {
        let normal = normal.normalize();

        Self { ray_t, origin, normal }
    }
}

impl Eq for Interaction { }

impl PartialOrd for Interaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ray_t.partial_cmp(&other.ray_t)
    }
}

impl Ord for Interaction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("invalid value for ray_t")
    }
}

#[cfg(test)]
mod test;