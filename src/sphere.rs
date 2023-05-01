use std::f64::EPSILON;

use float_eq::float_eq;

use crate::{ray::Ray, tuple::Tuple};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sphere;

impl Sphere {
    pub fn new() -> Self {
        Sphere
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction * ray.direction;
        let b = 2.0 * (ray.direction * sphere_to_ray);
        let c = sphere_to_ray * sphere_to_ray - 1.0;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = Intersection::new(
                (-b - f64::sqrt(discriminant)) / (2.0 * a),
                Object::Sphere(*self),
            );
            let t2 = Intersection::new(
                (-b + f64::sqrt(discriminant)) / (2.0 * a),
                Object::Sphere(*self),
            );

            vec![t1, t2]
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Object {
    Sphere(Sphere),
}

#[derive(Clone, Debug)]
pub struct Intersection {
    pub value: f64,
    pub object: Object,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        float_eq!(self.value, other.value, abs <= EPSILON) && self.object == other.object
    }
}

impl Eq for Intersection {}

impl Intersection {
    pub fn new(value: f64, object: Object) -> Self {
        Self { value, object }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::EPSILON;

    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn test_create_sphere() {
        Sphere::new();
    }

    #[test]
    fn test_intersect() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].value, 5.0);
        assert_eq!(xs[1].value, 5.0);
    }

    #[test]
    fn test_intersect_miss() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_ray_inside_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].value, -1.0);
        assert_eq!(xs[1].value, 1.0);
    }

    #[test]
    fn test_ray_behind_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].value, -6.0);
        assert_eq!(xs[1].value, -4.0);
    }

    #[test]
    fn test_intersection_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, Object::Sphere(s));

        assert_float_eq!(i.value, 3.5, abs <= EPSILON);
        assert_eq!(i.object, Object::Sphere(s));
    }

    #[test]
    fn test_intersect_set_object() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, Object::Sphere(s));
        assert_eq!(xs[1].object, Object::Sphere(s));
    }
}
