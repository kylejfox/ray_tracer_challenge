use crate::{
    intersections::{Intersection, Intersections},
    matrices::{CastingMatrixError, Matrix, NoInverseError, IDENTITY},
    rays::Ray,
    Point, Vector,
};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    transform: Matrix,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IntersectingSphereError {
    NoInverse,
    CastingMatrix,
}

impl From<NoInverseError> for IntersectingSphereError {
    fn from(value: NoInverseError) -> Self {
        IntersectingSphereError::NoInverse
    }
}

impl From<CastingMatrixError> for IntersectingSphereError {
    fn from(value: CastingMatrixError) -> Self {
        IntersectingSphereError::CastingMatrix
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: IDENTITY,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn intersect(&self, ray: Ray) -> Result<Intersections, IntersectingSphereError> {
        let ray2 = ray.transformed(self.transform.inverse()?)?;
        let sphere_to_ray = ray2.origin() - Point::new(0.0, 0.0, 0.0);

        let a = Vector::dot(ray2.direction(), ray2.direction());
        let b = 2.0 * Vector::dot(ray2.direction(), sphere_to_ray);
        let c = Vector::dot(sphere_to_ray, sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Ok(Intersections::new(vec![]))
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            Ok(Intersections::new(vec![
                Intersection::new(t1, self),
                Intersection::new(t2, self),
            ]))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        rays::Ray,
        transformations::{scaling, translation},
        Point, Vector,
    };

    use super::*;

    #[test]
    fn intersect_twice() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).expect("intersecting sphere error");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 6.0);
    }

    #[test]
    fn tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).expect("intersecting sphere error");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 5.0);
        assert_eq!(xs[1].t(), 5.0);
    }

    #[test]
    fn miss() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).expect("intersecting sphere error");
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn from_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).expect("intersecting sphere error");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), -1.0);
        assert_eq!(xs[1].t(), 1.0);
    }

    #[test]
    fn behind() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).expect("intersecting sphere error");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), -6.0);
        assert_eq!(xs[1].t(), -4.0);
    }

    #[test]
    fn intersection_sets_object() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).expect("intersecting sphere error");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object(), &s);
        assert_eq!(xs[1].object(), &s);
    }

    #[test]
    fn default_transform() {
        let s = Sphere::new();
        assert_eq!(s.transform, IDENTITY);
    }

    #[test]
    fn change_transform() {
        let mut s = Sphere::new();
        let t = translation(2.0, 3.0, 4.0);
        s.set_transform(t);
        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersect_scaled() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r).expect("intersecting sphere error");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 3.0);
        assert_eq!(xs[1].t(), 7.0);
    }

    #[test]
    fn intersect_translated() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(r).expect("intersecting sphere error");
        assert_eq!(xs.len(), 0);
    }
}
