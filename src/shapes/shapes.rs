use crate::rt::ray::{Intersection, Ray};
use crate::*;

pub trait Shape {
    fn intersect(&self, ray: &Ray, bound: &Bound<1>) -> Option<Intersection>;
}
