use super::*;

type Subject = Material;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_material_that_has_a_color() {
        let purple = Vector::new(0.5, 0.0, 0.5);
        let subject = Subject::new(purple);

        assert_eq!(subject.color, purple);
    }
}