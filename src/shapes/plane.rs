use crate::*;

pub struct Plane {
    pub point: Vector3,
    pub normal: Vector3,
}

impl Plane {
    pub fn new(point: Vector3, normal: Vector3) -> Plane {
        let normal = normal.normalize();
        Plane { point, normal }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray, bound: &Interval) -> Option<Intersection> {
        let distance = (self.point - ray.o).dot(&self.normal) / ray.d.dot(&self.normal);
        if !bound.contains(&Vector1::new([distance])) {
            return None;
        }

        let point = ray.at(distance);
        let normal = self.normal;
        Some(Intersection {
            distance,
            point,
            normal,
        })
    }
}
