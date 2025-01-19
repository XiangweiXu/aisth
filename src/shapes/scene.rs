use crate::lm::transform::Transform;
use crate::*;

pub struct Scene {
    pub shapes: Vec<Box<dyn Shape>>,
    pub transforms: Vec<Transform>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { shapes: Vec::new() }
    }

    pub fn push(
        mut self,
        shape: impl Shape,
        translation: &Vector3,
        rotation: &Vector3,
        scale: &Vector3,
    ) {
        let to_world = Transform::translate(translation)
            * Transform::rotate_x(rotation[0])
            * Transform::rotate_y(rotation[1])
            * Transform::rotate_z(rotation[2])
            * Transform::scale(scale[0], scale[1], scale[2]);
        self
    }
}

impl Shape for Scene {
    fn intersect(&self, ray: &Ray, bound: &Interval) -> Option<Intersection> {}
}
