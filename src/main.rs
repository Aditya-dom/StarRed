mod vector;
mod ray;
mod image;
mod film;
mod camera;
mod sphere;
mod interaction;
mod light;
mod material;
mod primitive;
mod scene;
mod integrator;

use vector::Vector;
use image::Image;
use film::Film;
use camera::Camera;
use sphere::Sphere;
use material::Material;
use primitive::Primitive;
use light::Light;
use scene::Scene;
use integrator::Integrator;

fn main() {
    let red = Vector::new(1.0, 0.2, 0.2);
    let sphere1 = Sphere::new(Vector::new(-1.0, 4.0, 15.0), 2.0);
    let mut primitive1 = Primitive::new(sphere1, Material::new(red));
    primitive1.material.reflectance = 1.0;

    let green = Vector::new(0.2, 1.0, 0.2);
    let sphere2 = Sphere::new(Vector::new(2.0, 2.0, 20.0), 5.0);
    let mut primitive2 = Primitive::new(sphere2, Material::new(green));
    primitive2.material.reflectance = 1.0;

    let purple = Vector::new(0.5, 0.0, 0.7);
    let sphere3 = Sphere::new(Vector::new(10.0, -1.0, 25.0), 3.0);
    let mut primitive3 = Primitive::new(sphere3, Material::new(purple));
    primitive3.material.reflectance = 0.7;

    let yellow = Vector::new(1.0, 1.0, 0.2);
    let sphere4 = Sphere::new(Vector::new(12.0, 4.0, 24.0), 2.0);
    let mut primitive4 = Primitive::new(sphere4, Material::new(yellow));
    primitive4.material.reflectance = 0.5;

    let blue = Vector::new(0.0, 0.2, 0.8);
    let sphere5 = Sphere::new(Vector::new(-5.0, -2.0, 12.0), 3.0);
    let mut primitive5 = Primitive::new(sphere5, Material::new(blue));
    primitive5.material.reflectance = 0.7;

    let pink = Vector::new(0.8, 0.5, 0.7);
    let sphere6 = Sphere::new(Vector::new(-1.0, -1.0, 11.0), 1.0);
    let mut primitive6 = Primitive::new(sphere6, Material::new(pink));
    primitive6.material.reflectance = 0.2;

    let white = Vector::new(1.0, 1.0, 1.0);
    let sphere7 = Sphere::new(Vector::new(-11.0, 6.0, 12.0), 4.0);
    let mut primitive7 = Primitive::new(sphere7, Material::new(white));
    primitive7.material.reflectance = 1.0;

    let black = Vector::new(0.0, 0.0, 0.0);
    let sphere8 = Sphere::new(Vector::new(6.0, -9.0, 12.0), 5.0);
    let mut primitive8 = Primitive::new(sphere8, Material::new(black));
    primitive8.material.reflectance = 1.0;

    let light1 = Light::new(Vector::new(-3.0, 12.0, -2.0), 300.0);
    let light2 = Light::new(Vector::new(12.0, 12.0, 22.0), 100.0);
    let light3 = Light::new(Vector::new(-5.0, 8.0, 30.0), 200.0);

    let scene = Scene::new(
        vec![primitive1, primitive2, primitive3, primitive4,
             primitive5, primitive6, primitive7, primitive8],

        vec![light1, light2, light3],
    );

    let origin = Vector::new(0.0, 0.0, 0.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let orientation = Vector::new(0.0, 1.0, 0.0);
    let image = Image::new(1920, 1080);
    let film = Film::new(1.92, 1.08, image);
    let camera = Camera::new(origin, direction, orientation, film);

    Integrator::new(camera).render(scene);
}