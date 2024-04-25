use super::*;
use crate::image::Image;

type Subject = Camera;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_camera_and_places_it_in_the_scene() {
        let origin = Vector::default();
        let direction = Vector::new(0.0, 0.0, 1.0);
        let orientation = Vector::new(0.0, 1.0, 0.0);

        let image = Image::new(200, 100);
        let film = Film::new(2.0, 1.0, image);

        let subject = Subject::new(origin, direction, orientation, film.clone());

        assert_eq!(subject.origin, origin);
        assert_eq!(subject.direction, direction);
        assert_eq!(subject.orientation, orientation);
        assert_eq!(subject.film, film);
    }

    #[test]
    fn it_normalizes_the_direction_and_orientation_vectors() {
        let origin = Vector::default();
        let direction = Vector::new(0.0, 0.0, 2.0);
        let orientation = Vector::new(0.0, 2.0, 0.0);

        let image = Image::new(200, 100);
        let film = Film::new(2.0, 1.0, image);

        let subject = Subject::new(origin, direction, orientation, film);

        assert_eq!(subject.direction, Vector::new(0.0, 0.0, 1.0));
        assert_eq!(subject.orientation, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn it_sets_vectors_that_span_left_to_right_and_top_to_bottom() {
        let origin = Vector::new(1.0, 2.0, 3.0);
        let direction = Vector::new(0.0, 0.0, 1.0);
        let orientation = Vector::new(0.0, 1.0, 0.0);

        let image = Image::new(200, 100);
        let film = Film::new(2.0, 1.0, image);

        let subject = Subject::new(origin, direction, orientation, film);

        assert_eq!(subject.left_to_right, Vector::new(2.0, 0.0, 0.0));
        assert_eq!(subject.top_to_bottom, Vector::new(0.0, -1.0, 0.0));
    }
}

mod trace_rays {
    use super::*;

    #[test]
    fn it_sets_the_color_of_each_pixel_from_the_closures_return_value() {
        let origin = Vector::new(1.0, 2.0, 3.0);
        let direction = Vector::new(0.0, 0.0, 1.0);
        let orientation = Vector::new(0.0, 1.0, 0.0);

        let image = Image::new(1, 1);
        let film = Film::new(2.0, 1.0, image);

        let mut subject = Subject::new(origin, direction, orientation, film);
        let purple = Vector::new(0.5, 0.0, 0.5);

        subject.trace_rays(|_| purple);

        assert_eq!(subject.film.image.get(0, 0), purple);
    }

    #[test]
    fn it_calls_the_closure_with_a_ray_for_each_pixel() {
        let origin = Vector::new(1.0, 2.0, 3.0);
        let direction = Vector::new(0.0, 0.0, 1.0);
        let orientation = Vector::new(0.0, 1.0, 0.0);

        let image = Image::new(1, 1);
        let film = Film::new(2.0, 1.0, image);

        let mut subject = Subject::new(origin, direction, orientation, film);

        subject.trace_rays(|ray| {
            let direction = Vector::new(0.0, 0.0, 1.0);

            assert_eq!(ray, Ray::new(origin, direction));

            Vector::default()
        });
    }
}

mod generate_ray {
    use super::*;

    #[test]
    fn it_generates_a_ray_that_travels_through_the_film_at_the_given_ratio() {
        let origin = Vector::new(1.0, 2.0, 3.0);
        let direction = Vector::new(0.0, 0.0, 1.0);
        let orientation = Vector::new(0.0, 1.0, 0.0);

        let image = Image::new(200, 100);
        let film = Film::new(2.0, 1.0, image);

        let subject = Subject::new(origin, direction, orientation, film);
        let ray = subject.generate_ray(0.3, 0.2);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, Vector::new(-0.4, 0.3, 1.0));
    }
}