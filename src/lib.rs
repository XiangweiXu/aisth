pub mod lm;
pub mod rt;
pub mod shapes;

pub use crate::lm::math::*;
pub use crate::lm::transform::*;
pub use crate::rt::ray::{Intersection, Ray};
pub use crate::shapes::{
    mesh::Mesh, plane::Plane, shape::Shape, sphere::Sphere, triangle::Triangle,
};
