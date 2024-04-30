use crate::scene::Scene;
use crate::camera::Camera;
use crate::vector::Vector;
use crate::ray::Ray;

pub struct Integrator {
    camera: Camera,
}

impl Integrator {
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }

    pub fn render(&mut self, scene: Scene) {
        self.camera.trace_rays(|ray| Self::li(ray, &scene, 0));

        self.camera.take_photograph("render.png");
    }

    fn li(ray: Ray, scene: &Scene, depth: u32) -> Vector {
        let intersection = scene.intersection(ray);

        // If the ray doesn't intersect, set a background based on the y param.
        if intersection.is_none() {
            let y = 0.7 - ray.direction.y.abs();
            let mut x = ray.direction.x / 2.0;
            if x < y { x = y }

            return Vector::new(x, x, y);
        }

        // Get details of the intersection, e.g. surface normal, primitive
        let (interaction, primitive) = intersection.unwrap();

        let total_radiance = scene.lights.iter().map(|light| {
            // Get the amount of light falling on the point of intersection.
            let (radiance, incident) = light.sample_li(&interaction);

            // Build a ray from the point of intersection to the light.
            let shadow_ray = Ray::new(interaction.origin, incident);

            // If the shadow ray intersects a primitive...
            if let Some((i, _)) = scene.intersection(shadow_ray) {

                // ... Don't add its radiance if the primitive occludes the light.
                if i.ray_t < (light.origin - shadow_ray.origin).length() {
                    return 0.0;
                }
            }

            // Calculate the proportion of light falling on the tilted surface.
            let cosine = incident.dot(interaction.normal).abs();

            radiance * cosine
        }).sum::<f64>();

        let reflectance = primitive.material.reflectance;

        let reflection_color = match reflectance > 0.0 || depth >= 100 {
            false => Vector::default(),
            true => {
                let cosine = ray.direction.dot(interaction.normal);
                let angle = ray.direction - interaction.normal * cosine * 2.0;

                let reflection_ray = Ray::new(interaction.origin, angle);

                Self::li(reflection_ray, scene, depth + 1) * reflectance
            },
        };

        (primitive.material.color + reflection_color) * total_radiance
    }
}

#[cfg(test)]
mod test;