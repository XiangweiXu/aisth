use crate::*;

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub vertices: [Vector3; 3],
    pub normals: [Vector3; 3],
}

impl Triangle {
    pub fn new(vertices: [Vector3; 3], normals: [Vector3; 3]) -> Triangle {
        Triangle { vertices, normals }
    }

    pub fn build(vertices: [Vector3; 3]) -> Triangle {
        let side_1 = vertices[1] - vertices[0];
        let side_2 = vertices[2] - vertices[0];
        let normal = side_1.cross(&side_2).normalize();
        let normals = [normal, normal, normal];
        Triangle { vertices, normals }
    }
}

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray, bound: &Interval) -> Option<Intersection> {
        let e_1 = self.vertices[1] - self.vertices[0];
        let e_2 = self.vertices[2] - self.vertices[0];
        let s = ray.o - self.vertices[0];
        let s_1 = ray.d.cross(&e_2);
        let s_2 = s.cross(&e_1);

        let det_recip = e_1.dot(&s_1).recip();

        let u = s_1.dot(&s) * det_recip;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let v = s_2.dot(&ray.d) * det_recip;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let distance = s_2.dot(&e_2) * det_recip;
        if !bound.contains(&Vector1::new([distance])) {
            return None;
        }

        let point = ray.at(distance);
        let normal =
            ((1.0 - u - v) * self.normals[0] + u * self.normals[1] + v * self.normals[2])
                .normalize();

        Some(Intersection {
            distance,
            point,
            normal,
        })
    }
}
