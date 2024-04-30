use super::*;
use crate::image::Image;
use crate::film::Film;
use crate::light::Light;
use crate::sphere::Sphere;
use crate::primitive::Primitive;
use crate::material::Material;

type Subject = Integrator;

fn camera() -> Camera {
    let origin = Vector::default();
    let direction = Vector::new(0.0, 0.0, 1.0);
    let orientation = Vector::new(0.0, 1.0, 0.0);
    let image = Image::new(200, 100);
    let film = Film::new(2.0, 1.0, image);

    Camera::new(origin, direction, orientation, film)
}

fn scene() -> Scene {
    let yellow = Vector::new(1.0, 1.0, 0.0);

    // A large sphere in the center.
    let sphere1 = Sphere::new(Vector::new(0.0, 0.0, 3.0), 1.0);
    let primitive1 = Primitive::new(sphere1, Material::new(yellow));

    // A smaller sphere to the right.
    let sphere2 = Sphere::new(Vector::new(0.5, 0.5, 2.0), 0.1);
    let primitive2 = Primitive::new(sphere2, Material::new(yellow));

    // This sphere is behind the light.
    let sphere3 = Sphere::new(Vector::new(1.5, 1.0, 1.0), 0.4);
    let primitive3 = Primitive::new(sphere3, Material::new(yellow));

    let light = Light::new(Vector::new(1.0, 1.0, 1.0), 2.0);

    Scene::new(vec![primitive1, primitive2, primitive3], vec![light])
}

mod new {
    use super::*;

    #[test]
    fn it_builds_the_integrator_with_a_camera() {
        let camera = camera();
        let subject = Subject::new(camera.clone());

        assert_eq!(subject.camera, camera);
    }
}

mod li {
    use super::*;

    #[test]
    fn it_returns_the_background_color_if_the_ray_doesnt_intersect() {
        let subject = Subject::new(camera());

        let scene = scene();
        let ray = Ray::new(Vector::default(), Vector::new(1.0, 0.0, 0.0));

        let color = Subject::li(ray, &scene, 0);

        assert_eq!(color, Vector::new(0.2, 0.2, 0.2));
    }

    #[test]
    fn it_returns_the_shaded_color_if_the_ray_intersects() {
        let subject = Subject::new(camera());

        let scene = scene();
        let ray = Ray::new(Vector::default(), Vector::new(0.0, 0.0, 1.0));

        let color = Subject::li(ray, &scene, 0);

        assert!(color.x > 0.3 && color.x < 0.4);
        assert!(color.y > 0.3 && color.y < 0.4);

        assert_eq!(color.z, 0.0);
    }

    #[test]
    fn it_returns_black_if_the_intersection_point_is_in_shadow() {
        let subject = Subject::new(camera());

        let scene = scene();
        let ray = Ray::new(Vector::default(), Vector::new(0.2, 0.2, 1.0));

        let color = Subject::li(ray, &scene, 0);

        assert_eq!(color, Vector::default());
    }

    #[test]
    fn it_does_not_cast_shadows_from_primitives_behind_the_light() {
        let subject = Subject::new(camera());

        let scene = scene();
        let ray = Ray::new(Vector::default(), Vector::new(-0.1, 0.0, 1.0));

        let color = Subject::li(ray, &scene, 0);

        assert!(color.x > 0.2 && color.x < 0.3);
        assert!(color.x > 0.2 && color.x < 0.3);

        assert_eq!(color.z, 0.0);
    }
}