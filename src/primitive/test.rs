use super::*;
use crate::vector::Vector;

type Subject = Primitive;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_primitive_that_combines_a_shape_with_a_material() {
        let sphere = Sphere::new(Vector::default(), 1.0);
        let aubergine = Material::new(Vector::new(0.5, 0.0, 0.5));

        let subject = Subject::new(sphere, aubergine);

        assert_eq!(subject.shape, sphere);
        assert_eq!(subject.material, aubergine);
    }
}