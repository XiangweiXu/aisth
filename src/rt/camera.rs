use crate::{
    rt::{film::Film, ray::Ray},
    *,
};

pub struct Camera {
    film: Film,
    position: Vector<3>,
    from_clip_space: Transform,
    to_world_space: Transform,
}

impl Camera {
    pub fn build(
        film: Film,
        position: Vector<3>,
        look: Vector<3>,
        fov_degrees: Float,
    ) -> Camera {
        let from_clip_space = Transform::perspective(fov_degrees, 1.0, 2.0).transpose();
        let to_world_space =
            Transform::look_at(&position, &look, &Vector::<3>::new([0.0, 1.0, 0.0]))
                .transpose();

        Camera {
            film,
            position,
            from_clip_space,
            to_world_space,
        }
    }

    pub fn cast_ray(&self, x: usize, y: usize) -> Ray {
        let offset = Vector::<2>::new([0.5, 0.5]);
        let mut ndc = (Vector::<2>::new([x as Float, y as Float]) + offset)
            / Vector::<2>::new([
                self.film.size().0 as Float,
                self.film.size().1 as Float,
            ]);
        ndc[1] = 1.0 - ndc[1];
        ndc = ndc * 2.0 - Vector::<2>::new([1.0, 1.0]);
        ndc[0] *= self.film.aspect();
        let clip = Vector::<4>::new([ndc[0], ndc[1], 0.0, 1.0]);
        let world = self.to_world_space.matrix() * self.from_clip_space.matrix() * clip;
        Ray::new(
            self.position,
            (Vector::<3>::new([world[0], world[1], world[2]]) - self.position)
                .normalize(),
        )
    }

    pub const fn film(&self) -> &Film {
        &self.film
    }
}
