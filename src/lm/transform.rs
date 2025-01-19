use crate::*;

pub struct Transform {
    matrix: SquareMatrix<4>,
    inversed: SquareMatrix<4>,
}

impl Transform {
    pub fn build(matrix: &SquareMatrix<4>) -> Transform {
        let inversed = matrix
            .inverse()
            .expect("No valid inverse for input matrix!");
        Transform {
            matrix: *matrix,
            inversed,
        }
    }

    pub const fn matrix(&self) -> SquareMatrix<4> {
        self.matrix
    }

    pub const fn inversed(&self) -> SquareMatrix<4> {
        self.inversed
    }

    pub const fn transpose(&self) -> Transform {
        Transform {
            matrix: self.inversed,
            inversed: self.matrix,
        }
    }

    pub fn translate(delta: &Vector<3>) -> Transform {
        let matrix = SquareMatrix::<4>::new([
            [1.0, 0.0, 0.0, delta[0]],
            [0.0, 1.0, 0.0, delta[1]],
            [0.0, 0.0, 1.0, delta[2]],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let inversed = SquareMatrix::<4>::new([
            [1.0, 0.0, 0.0, delta[0]],
            [0.0, 1.0, 0.0, delta[1]],
            [0.0, 0.0, 1.0, delta[2]],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Transform { matrix, inversed }
    }

    pub fn scale(x: Float, y: Float, z: Float) -> Transform {
        let matrix = SquareMatrix::<4>::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let inversed = SquareMatrix::<4>::new([
            [x.recip(), 0.0, 0.0, 0.0],
            [0.0, y.recip(), 0.0, 0.0],
            [0.0, 0.0, z.recip(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Transform { matrix, inversed }
    }

    pub fn rotate_x(degrees: Float) -> Transform {
        let sin_theta = degrees.to_radians().sin();
        let cos_theta = degrees.to_radians().cos();
        let matrix = SquareMatrix::<4>::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_theta, -sin_theta, 0.0],
            [0.0, sin_theta, cos_theta, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Transform {
            matrix,
            inversed: matrix.transpose(),
        }
    }

    pub fn rotate_y(degrees: Float) -> Transform {
        let sin_theta = degrees.to_radians().sin();
        let cos_theta = degrees.to_radians().cos();
        let matrix = SquareMatrix::<4>::new([
            [cos_theta, 0.0, sin_theta, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin_theta, 0.0, cos_theta, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Transform {
            matrix,
            inversed: matrix.transpose(),
        }
    }

    pub fn rotate_z(degrees: Float) -> Transform {
        let sin_theta = degrees.to_radians().sin();
        let cos_theta = degrees.to_radians().cos();
        let matrix = SquareMatrix::<4>::new([
            [cos_theta, -sin_theta, 0.0, 0.0],
            [sin_theta, cos_theta, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Transform {
            matrix,
            inversed: matrix.transpose(),
        }
    }

    pub fn look_at(position: &Vector<3>, look: &Vector<3>, up: &Vector<3>) -> Transform {
        let direction = (look - position).normalize();

        if up.normalize().cross(&direction).magnitude() == 0.0 {
            panic!("look at!")
        }

        let right = up.normalize().cross(&direction).normalize();
        let new_up = &direction.cross(&right);

        let to_world = SquareMatrix::<4>::new([
            [right[0], new_up[0], direction[0], position[0]],
            [right[1], new_up[1], direction[1], position[1]],
            [right[2], new_up[2], direction[2], position[2]],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let to_camera = to_world.inverse().unwrap();

        Transform {
            matrix: to_camera,
            inversed: to_world,
        }
    }

    pub fn orthographic(z_near: Float, z_far: Float) -> Transform {
        &Self::scale(1.0, 1.0, (z_far - z_near).recip())
            * &Self::translate(&Vector::<3>::new([0.0, 0.0, -z_near]))
    }

    pub fn perspective(fov_degrees: Float, z_near: Float, z_far: Float) -> Transform {
        let perspective = SquareMatrix::<4>::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [
                0.0,
                0.0,
                z_far / (z_far - z_near),
                -z_far * z_near / (z_far - z_near),
            ],
            [0.0, 0.0, 1.0, 0.0],
        ]);

        let tan_recip = Float::tan(fov_degrees.to_radians() * 0.5).recip();
        &Self::scale(tan_recip, tan_recip, 1.0) * &Self::build(&perspective)
    }
}

impl std::ops::Mul<&Transform> for &Transform {
    type Output = Transform;

    fn mul(self, rhs: &Transform) -> Self::Output {
        Transform {
            matrix: self.matrix * rhs.matrix,
            inversed: rhs.inversed * self.inversed,
        }
    }
}

impl std::ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: &Transform) -> Self::Output {
        Transform {
            matrix: self.matrix * rhs.matrix,
            inversed: rhs.inversed * self.inversed,
        }
    }
}
