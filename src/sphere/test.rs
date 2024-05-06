use super::*;

type Subject = Sphere;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_sphere() {
        let origin = Vector::new(1.0, 2.0, 3.0);
        let subject = Subject::new(origin, 1.5);

        assert_eq!(subject.origin, origin);
        assert_eq!(subject.radius, 1.5);
    }
}

mod intersection {
    use super::*;

    fn intersection(ray_y_direction: f64, sphere_radius: f64) -> Option<Interaction> {
        let origin = Vector::new(2.0, 0.0, 0.0);
        let sphere = Subject::new(origin, sphere_radius);

        let origin = Vector::new(0.0, 0.0, 0.0);
        let direction = Vector::new(1.0, ray_y_direction, 0.0);
        let ray = Ray::new(origin, direction);

        sphere.intersection(ray)
    }

    #[test]
    fn it_returns_none_if_the_ray_does_not_intersect_the_sphere() {
        assert_eq!(intersection(0.6, 1.0), None);
    }

    #[test]
    fn it_returns_the_nearest_interaction_if_the_ray_intersects_the_sphere() {
        let interaction = intersection(0.5, 1.0)
            .expect("The ray should have intersected the front of the sphere.");

        assert_eq!(interaction.ray_t, 1.2);
        assert_eq!(interaction.normal, Vector::new(-0.8, 0.6, 0.0));
    }

    #[test]
    fn it_ignores_interactions_at_the_origin_of_the_ray() {
        let interaction = intersection(0.5, 2.0)
            .expect("The ray should have intersected the back of the sphere.");

        assert_eq!(interaction.ray_t, 3.2);
    }

    #[test]
    fn it_ignores_interactions_epsilon_along_the_ray_to_counter_floating_precision_errors() {
        let interaction = intersection(0.5, 1.99999999999)
            .expect("The ray should have intersected the back of the sphere.");

        assert_eq!(interaction.ray_t > 3.1, true);
        assert_eq!(interaction.ray_t < 3.3, true);
    }
}