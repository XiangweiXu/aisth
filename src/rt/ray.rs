use crate::*;

pub struct Ray {
    pub o: Vector<3>,
    pub d: Vector<3>,
}

impl Ray {
    pub fn new(o: Vector<3>, d: Vector<3>) -> Ray {
        Ray { o, d }
    }

    pub fn at(&self, t: Float) -> Vector<3> {
        self.o + self.d * t
    }
}

pub struct Intersection {
    pub distance: Float,
    pub point: Vector<3>,
    pub normal: Vector<3>,
}
