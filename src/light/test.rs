use super::*;

type Subject = Light;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_light_with_origin_and_power() {
        let origin = Vector::new(1.0, 2.0, 3.0);
        let power = 5.0;

        let subject = Subject::new(origin, power);

        assert_eq!(subject.origin, origin);
        assert_eq!(subject.power, power);
    }
}

mod sample_li {
    use super::*;

    #[test]
    fn it_calculates_radiance_by_dividing_the_lights_power_by_the_radius_squared() {
        let origin = Vector::new(1.0, 2.0, 3.0);
        let subject = Subject::new(origin, 5.0);

        let origin = Vector::new(4.0, 5.0, 6.0);
        let interaction = Interaction::new(0.0, origin, Vector::default());

        let (radiance, _) = subject.sample_li(&interaction);

        assert_eq!(radiance, 5.0 / 27.0);
    }

    #[test]
    fn it_builds_a_vector_pointing_towards_the_light() {
        let origin = Vector::new(1.0, 2.0, 3.0);
        let subject = Subject::new(origin, 5.0);

        let origin = Vector::new(4.0, 5.0, 6.0);
        let interaction = Interaction::new(0.0, origin, Vector::default());

        let (_, incident) = subject.sample_li(&interaction);

        assert_eq!(incident, Vector::new(-3.0, -3.0, -3.0).normalize());
    }
}