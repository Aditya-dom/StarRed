use super::*;
use crate::vector::Vector;
use crate::sphere::Sphere;
use crate::material::Material;

type Subject = Scene;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_scene_with_primitives_and_lights() {
        let sphere = Sphere::new(Vector::default(), 1.0);
        let aubergine = Material::new(Vector::new(0.5, 0.0, 0.5));
        let primitive = Primitive::new(sphere, aubergine);
        let light = Light::new(Vector::default(), 5.0);

        let subject = Subject::new(vec![primitive], vec![light]);

        assert_eq!(subject.primitives, &[primitive]);
        assert_eq!(subject.lights, &[light]);
    }
}

mod intersection {
    use super::*;

    fn subject() -> Subject {
        let aubergine = Material::new(Vector::new(0.5, 0.0, 0.5));

        let sphere1 = Sphere::new(Vector::new(2.0, 0.0, 0.0), 1.0);
        let primitive1 = Primitive::new(sphere1, aubergine);

        let sphere2 = Sphere::new(Vector::new(10.0, 0.0, 0.0), 8.0);
        let primitive2 = Primitive::new(sphere2, aubergine);

        Scene::new(vec![primitive1, primitive2], vec![])
    }

    #[test]
    fn it_returns_none_if_the_ray_does_not_intersect_any_primitive() {
        let direction = Vector::new(1.0, 2.0, 0.0);
        let ray = Ray::new(Vector::default(), direction);

        assert_eq!(subject().intersection(ray), None);
    }

    #[test]
    fn it_returns_the_interaction_and_primitive_for_the_shape_at_the_front() {
        let subject = subject();

        let direction = Vector::new(1.0, 0.5, 0.0);
        let ray = Ray::new(Vector::default(), direction);

        let (interaction, primitive) = subject.intersection(ray)
            .expect("The ray should have intersected the sphere at the front.");

        assert_eq!(interaction.normal, Vector::new(-0.8, 0.6, 0.0));
        assert_eq!(primitive.shape.radius, 1.0);
    }

    #[test]
    fn it_returns_the_interaction_and_primitive_for_the_shape_at_the_back() {
        let subject = subject();

        let direction = Vector::new(1.0, 0.6, 0.0);
        let ray = Ray::new(Vector::default(), direction);

        let (_, primitive) = subject.intersection(ray)
            .expect("The ray should have intersected the sphere at the back.");

        assert_eq!(primitive.shape.radius, 8.0);
    }
}