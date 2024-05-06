use super::*;

type Subject = Vector;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_vector_with_x_y_and_z_coordinates() {
        let subject = Subject::new(1.0, 2.0, 3.0);

        assert_eq!(subject.x, 1.0);
        assert_eq!(subject.y, 2.0);
        assert_eq!(subject.z, 3.0);
    }
}

mod addition {
    use super::*;

    #[test]
    fn it_adds_the_vectors_components() {
        let a = Subject::new(1.0, 2.0, 3.0);
        let b = Subject::new(4.0, 5.0, 6.0);

        let subject = a + b;

        assert_eq!(subject, Subject::new(5.0, 7.0, 9.0));
    }
}

mod subtraction {
    use super::*;

    #[test]
    fn it_subtracts_the_vectors_components() {
        let a = Subject::new(5.0, 5.0, 5.0);
        let b = Subject::new(1.0, 2.0, 3.0);

        let subject = a - b;

        assert_eq!(subject, Subject::new(4.0, 3.0, 2.0));
    }
}

mod scaling {
    use super::*;

    #[test]
    fn it_multiplies_the_vectors_components_by_a_scalar() {
        let vector = Subject::new(1.0, 2.0, 3.0);
        let subject = vector * 4.0;

        assert_eq!(subject, Subject::new(4.0, 8.0, 12.0));
    }
}

mod cross_product {
    use super::*;

    #[test]
    fn it_calculates_the_cross_product_of_two_vectors() {
        // Example from:
        // https://www.symbolab.com/solver/vector-cross-product-calculator

        let a = Subject::new(1.0, 2.0, 3.0);
        let b = Subject::new(1.0, 5.0, 7.0);

        let subject = a * b;

        assert_eq!(subject.x, -1.0);
        assert_eq!(subject.y, -4.0);
        assert_eq!(subject.z, 3.0);
    }
}

mod division {
    use super::*;

    #[test]
    fn it_divides_the_vectors_components_by_a_divisor() {
        let vector = Subject::new(1.0, 2.0, 3.0);
        let subject = vector / 4.0;

        assert_eq!(subject, Subject::new(0.25, 0.5, 0.75));
    }
}

mod negation {
    use super::*;

    #[test]
    fn it_negates_the_vectors_components() {
        let subject = Subject::new(1.0, 2.0, 3.0);

        assert_eq!(-subject, Subject::new(-1.0, -2.0, -3.0));
    }
}

mod length {
    use super::*;

    #[test]
    fn it_returns_the_length_of_the_vector() {
        let subject = Subject::new(1.0, 2.0, 3.0);

        assert_eq!(subject.length(), f64::sqrt(14.0));
    }
}

mod normalize {
    use super::*;

    #[test]
    fn it_divides_each_component_by_the_vectors_length() {
        let vector = Subject::new(1.0, 2.0, 3.0);
        let subject = vector.normalize();

        let l = vector.length();
        assert_eq!(subject, Subject::new(1.0 / l, 2.0 / l, 3.0 / l));
    }
}

mod lerp {
    use super::*;

    #[test]
    fn it_linearly_interpolates_between_two_vectors() {
        let a = Subject::new(1.0, 2.0, 3.0);
        let b = Subject::new(4.0, 5.0, 6.0);

        let subject = a.lerp(b, 0.5);

        assert_eq!(subject, Subject::new(2.5, 3.5, 4.5));
    }
}

mod dot {
    use super::*;

    #[test]
    fn it_returns_the_dot_product_of_two_vectors() {
        let a = Subject::new(1.0, 2.0, 3.0);
        let b = Subject::new(4.0, 5.0, 6.0);

        assert_eq!(a.dot(b), 32.0);
    }
}