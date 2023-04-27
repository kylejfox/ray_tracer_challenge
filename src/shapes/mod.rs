use crate::{
    intersections::{Intersection, Intersections},
    material::Material,
    matrices::{NoInverseError, Transform, IDENTITY},
    rays::Ray,
};

use std::{cell::RefCell, fmt::Debug};

pub mod spheres;

#[cfg(test)]
thread_local! {
    static SAVED_RAY: RefCell<Ray> = RefCell::new(Ray::default());
}

pub trait Model: Debug {
    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64>;

    fn dynamic_clone(&self) -> Box<dyn Model>;

    fn dynamic_eq(&self, other: &dyn Model) -> bool;
}

pub struct Shape {
    transform: Transform,
    inverse: Transform,
    pub material: Material,
    pub model: Box<dyn Model>,
}

impl Shape {
    pub fn new(model: impl Model) -> Self {
        Shape {
            transform: IDENTITY,
            inverse: IDENTITY,
            material: Material::default(),
            model: Box::new(model),
        }
    }

    pub fn set_transform(&mut self, transform: Transform) -> Result<(), NoInverseError> {
        let inverse = transform.inverse()?;
        self.transform = transform;
        self.inverse = inverse;
        Ok(())
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let local_ray = ray.transformed(&self.inverse);
        Intersections::new(
            self.model
                .local_intersect(&local_ray)
                .into_iter()
                .map(|t| Intersection::new(t, self))
                .collect(),
        )
    }
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        Shape {
            transform: self.transform.clone(),
            inverse: self.inverse.clone(),
            material: self.material.clone(),
            model: self.model.dynamic_clone(),
        }
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
            && self.inverse == other.inverse
            && self.material == other.material
            && self.model.dynamic_eq(other.model.as_ref())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        matrices::IDENTITY,
        transformations::{scaling, translation},
        Point, Vector,
    };

    use super::*;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
    struct TestModel;

    impl Model for TestModel {
        fn local_intersect(&self, local_ray: &'_ Ray) -> Vec<f64> {
            #[cfg(test)]
            {
                SAVED_RAY.with(|saved_ray| saved_ray.replace(local_ray.clone()));
            }
            vec![]
        }

        fn dynamic_clone(&self) -> Box<dyn Model> {
            Box::new(Self)
        }

        fn dynamic_eq(&self, other: &dyn Model) -> bool {
            todo!()
        }
    }

    #[test]
    fn default_transform() {
        let s = Shape::new(TestModel);
        assert_eq!(s.transform, IDENTITY);
    }

    #[test]
    fn assigning_transform() {
        let mut s = Shape::new(TestModel);
        s.set_transform(translation(2.0, 3.0, 4.0)).unwrap();
        assert_eq!(s.transform, translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn default_material() {
        let s = Shape::new(TestModel);
        let m = Material::default();
        assert_eq!(s.material, m);
    }

    #[test]
    fn assigning_material() {
        let mut s = Shape::new(TestModel);
        let m = Material {
            ambient: 1.0,
            ..Material::default()
        };
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn intersect_scaled_shape() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Shape::new(TestModel);
        s.set_transform(scaling(2.0, 2.0, 2.0)).unwrap();
        _ = s.intersect(&r);
        assert_eq!(s.saved_ray.borrow().origin, Point::new(0.0, 0.0, -2.5));
        assert_eq!(s.saved_ray.borrow().direction, Vector::new(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersect_translated_shape() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Shape::new(TestModel);
        s.set_transform(translation(5.0, 0.0, 0.0)).unwrap();
        _ = s.intersect(&r);
        assert_eq!(s.saved_ray.borrow().origin, Point::new(-5.0, 0.0, -5.0));
        assert_eq!(s.saved_ray.borrow().direction, Vector::new(0.0, 0.0, 1.0));
    }
}
