use crate::{
    intersections::{Intersection, Intersections},
    rays::Ray,
    Point, Vector,
};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {}
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let origin = ray.origin();
        let direction = ray.direction();
        let sphere_to_ray = origin - Point::new(0.0, 0.0, 0.0);

        let a = Vector::dot(direction, direction);
        let b = 2.0 * Vector::dot(direction, sphere_to_ray);
        let c = Vector::dot(sphere_to_ray, sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Intersections::new(vec![])
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            Intersections::new(vec![
                Intersection::new(t1, self),
                Intersection::new(t2, self),
            ])
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{rays::Ray, Point, Vector};

    use super::*;

    #[test]
    fn intersect_twice() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 6.0);
    }

    #[test]
    fn tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 5.0);
        assert_eq!(xs[1].t(), 5.0);
    }

    #[test]
    fn miss() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn from_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), -1.0);
        assert_eq!(xs[1].t(), 1.0);
    }

    #[test]
    fn behind() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), -6.0);
        assert_eq!(xs[1].t(), -4.0);
    }

    #[test]
    fn intersection_sets_object() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object(), &s);
        assert_eq!(xs[1].object(), &s);
    }
}
