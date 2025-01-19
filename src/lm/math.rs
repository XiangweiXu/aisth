use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

pub type Float = f64;

pub fn sub_mul_pair(a: Float, b: Float, c: Float, d: Float) -> Float {
    let cd = c * d;
    let sub_mul_pair = a.mul_add(b, -cd);
    let error = (-c).mul_add(d, cd);
    sub_mul_pair + error
}

pub fn add_mul_pair(a: Float, b: Float, c: Float, d: Float) -> Float {
    let cd = c * d;
    let add_mul_pair = a.mul_add(b, cd);
    let error = c.mul_add(d, -cd);
    add_mul_pair + error
}

#[derive(Copy, Clone, Debug)]
pub struct SquareMatrix<const N: usize> {
    pub m: [[Float; N]; N],
}

impl<const N: usize> SquareMatrix<N> {
    pub fn new(m: [[Float; N]; N]) -> SquareMatrix<N> {
        SquareMatrix::<N> { m }
    }

    pub fn identity() -> SquareMatrix<N> {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                if i == j {
                    m[i][j] = 1.0;
                } else {
                    m[i][j] = 0.0;
                }
            }
        }

        SquareMatrix::<N> { m }
    }

    pub fn zero() -> SquareMatrix<N> {
        SquareMatrix::<N> { m: [[0.0; N]; N] }
    }

    pub fn transpose(&self) -> SquareMatrix<N> {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self[j][i];
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl SquareMatrix<4> {
    pub fn inverse(&self) -> Option<SquareMatrix<4>> {
        let s_0 = sub_mul_pair(self[0][0], self[1][1], self[1][0], self[0][1]);
        let s_1 = sub_mul_pair(self[0][0], self[1][2], self[1][0], self[0][2]);
        let s_2 = sub_mul_pair(self[0][0], self[1][3], self[1][0], self[0][3]);
        let s_3 = sub_mul_pair(self[0][1], self[1][2], self[1][1], self[0][2]);
        let s_4 = sub_mul_pair(self[0][1], self[1][3], self[1][1], self[0][3]);
        let s_5 = sub_mul_pair(self[0][2], self[1][3], self[1][2], self[0][3]);

        let c_0 = sub_mul_pair(self[2][0], self[3][1], self[3][0], self[2][1]);
        let c_1 = sub_mul_pair(self[2][0], self[3][2], self[3][0], self[2][2]);
        let c_2 = sub_mul_pair(self[2][0], self[3][3], self[3][0], self[2][3]);
        let c_3 = sub_mul_pair(self[2][1], self[3][2], self[3][1], self[2][2]);
        let c_4 = sub_mul_pair(self[2][1], self[3][3], self[3][1], self[2][3]);
        let c_5 = sub_mul_pair(self[2][2], self[3][3], self[3][2], self[2][3]);

        let determinant = inner_product::<6>(
            [s_0, -s_1, s_2, s_3, s_5, -s_4],
            [c_5, c_4, c_3, c_2, c_0, c_1],
        );
        if determinant == 0.0 {
            return None;
        }

        let s = determinant.recip();

        let m = [
            [
                s * inner_product::<3>(
                    [self[1][1], self[1][3], -self[1][2]],
                    [c_5, c_3, c_4],
                ),
                s * inner_product::<3>(
                    [-self[0][1], self[0][2], -self[0][3]],
                    [c_5, c_4, c_3],
                ),
                s * inner_product::<3>(
                    [self[3][1], self[3][3], -self[3][2]],
                    [s_5, s_3, s_4],
                ),
                s * inner_product::<3>(
                    [-self[2][1], self[2][2], -self[2][3]],
                    [s_5, s_4, s_3],
                ),
            ],
            [
                s * inner_product::<3>(
                    [-self[1][0], self[1][2], -self[1][3]],
                    [c_5, c_2, c_1],
                ),
                s * inner_product::<3>(
                    [self[0][0], self[0][3], -self[0][2]],
                    [c_5, c_1, c_2],
                ),
                s * inner_product::<3>(
                    [-self[3][0], self[3][2], -self[3][3]],
                    [s_5, s_2, s_1],
                ),
                s * inner_product::<3>(
                    [self[2][0], self[2][3], -self[2][2]],
                    [s_5, s_1, s_2],
                ),
            ],
            [
                s * inner_product::<3>(
                    [self[1][0], self[1][3], -self[1][1]],
                    [c_4, c_0, c_2],
                ),
                s * inner_product::<3>(
                    [-self[0][0], self[0][1], -self[0][3]],
                    [c_4, c_2, c_0],
                ),
                s * inner_product::<3>(
                    [self[3][0], self[3][3], -self[3][1]],
                    [s_4, s_0, s_2],
                ),
                s * inner_product::<3>(
                    [-self[2][0], self[2][1], -self[2][3]],
                    [s_4, s_2, s_0],
                ),
            ],
            [
                s * inner_product::<3>(
                    [-self[1][0], self[1][1], -self[1][2]],
                    [c_3, c_1, c_0],
                ),
                s * inner_product::<3>(
                    [self[0][0], self[0][2], -self[0][1]],
                    [c_3, c_0, c_1],
                ),
                s * inner_product::<3>(
                    [-self[3][0], self[3][1], -self[3][2]],
                    [s_3, s_1, s_0],
                ),
                s * inner_product::<3>(
                    [self[2][0], self[2][2], -self[2][1]],
                    [s_3, s_0, s_1],
                ),
            ],
        ];

        Some(SquareMatrix::<4> { m })
    }
}

impl<const N: usize> Add<SquareMatrix<N>> for SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn add(self, rhs: SquareMatrix<N>) -> Self::Output {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self[i][j] + rhs[i][j];
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Add<&SquareMatrix<N>> for &SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn add(self, rhs: &SquareMatrix<N>) -> Self::Output {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self[i][j] + rhs[i][j];
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Sub<SquareMatrix<N>> for SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn sub(self, rhs: SquareMatrix<N>) -> Self::Output {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self[i][j] - rhs[i][j];
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Sub<&SquareMatrix<N>> for &SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn sub(self, rhs: &SquareMatrix<N>) -> Self::Output {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self[i][j] - rhs[i][j];
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Mul<SquareMatrix<N>> for SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn mul(self, rhs: SquareMatrix<N>) -> Self::Output {
        let t = rhs.transpose();
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self[i].iter().zip(t[j]).map(|(c1, c2)| c1 * c2).sum();
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Mul<&SquareMatrix<N>> for &SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn mul(self, rhs: &SquareMatrix<N>) -> Self::Output {
        let t = rhs.transpose();
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self[i].iter().zip(t[j]).map(|(c1, c2)| c1 * c2).sum();
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Mul<Vector<N>> for SquareMatrix<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        let mut v = [0.0; N];
        for i in 0..N {
            v[i] = self[i].iter().zip(rhs.v).map(|(c1, c2)| c1 * c2).sum();
        }

        Vector::<N> { v }
    }
}

impl<const N: usize> Mul<&Vector<N>> for &SquareMatrix<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: &Vector<N>) -> Self::Output {
        let mut v = [0.0; N];
        for i in 0..N {
            v[i] = self[i].iter().zip(rhs.v).map(|(c1, c2)| c1 * c2).sum();
        }

        Vector::<N> { v }
    }
}

impl<const N: usize> Mul<Float> for SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn mul(self, rhs: Float) -> Self::Output {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self[i][j] * rhs;
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Mul<Float> for &SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn mul(self, rhs: Float) -> Self::Output {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self[i][j] * rhs;
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Mul<SquareMatrix<N>> for Float {
    type Output = SquareMatrix<N>;

    fn mul(self, rhs: SquareMatrix<N>) -> Self::Output {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self * rhs[i][j];
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Mul<&SquareMatrix<N>> for Float {
    type Output = SquareMatrix<N>;

    fn mul(self, rhs: &SquareMatrix<N>) -> Self::Output {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self * rhs[i][j];
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Div<Float> for SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn div(self, rhs: Float) -> Self::Output {
        let recip = rhs.recip();
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self[i][j] * recip;
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Div<Float> for &SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn div(self, rhs: Float) -> Self::Output {
        let inv = rhs.recip();
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self[i][j] * inv;
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Neg for SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn neg(self) -> Self::Output {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = -self[i][j];
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Neg for &SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn neg(self) -> Self::Output {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = -self[i][j];
            }
        }

        SquareMatrix::<N> { m }
    }
}

impl<const N: usize> Index<usize> for SquareMatrix<N> {
    type Output = [Float; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}

impl<const N: usize> fmt::Display for SquareMatrix<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("[ [ ");
        for i in 0..N {
            for j in 0..N {
                s.push_str(&self.m[i][j].to_string());
                if j < N - 1 {
                    s.push(',');
                } else {
                    s.push_str(" ]");
                }
            }
            if i < N - 1 {
                s.push_str(" , [ ");
            }
        }
        s.push_str(" ]");
        write!(f, "{s}")
    }
}

pub type Matrix22 = SquareMatrix<2>;
pub type Matrix33 = SquareMatrix<3>;
pub type Matrix44 = SquareMatrix<4>;

#[derive(Copy, Clone, Debug)]
pub struct Vector<const N: usize> {
    pub v: [Float; N],
}

impl<const N: usize> Vector<N> {
    pub fn new(v: [Float; N]) -> Vector<N> {
        Vector::<N> { v }
    }

    pub const fn zero() -> Vector<N> {
        Vector::<N> { v: [0.0; N] }
    }

    pub fn dot(&self, rhs: &Vector<N>) -> Float {
        self.v.iter().zip(rhs.v).map(|(c1, c2)| c1 * c2).sum()
    }

    pub fn magnitude(&self) -> Float {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Vector<N> {
        self / self.magnitude()
    }
}

impl Vector<3> {
    pub fn cross(&self, rhs: &Vector<3>) -> Vector<3> {
        Vector::<3> {
            v: [
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0],
            ],
        }
    }
}

impl<const N: usize> Add<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn add(self, rhs: Vector<N>) -> Self::Output {
        let v = self
            .v
            .iter()
            .zip(rhs.v)
            .map(|(c1, c2)| c1 + c2)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Add<&Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn add(self, rhs: &Vector<N>) -> Self::Output {
        let v = self
            .v
            .iter()
            .zip(rhs.v)
            .map(|(c1, c2)| c1 + c2)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Sub<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn sub(self, rhs: Vector<N>) -> Self::Output {
        let v = self
            .v
            .iter()
            .zip(rhs.v)
            .map(|(c1, c2)| c1 - c2)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Sub<&Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn sub(self, rhs: &Vector<N>) -> Self::Output {
        let v = self
            .v
            .iter()
            .zip(rhs.v)
            .map(|(c1, c2)| c1 - c2)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Mul<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        let v = self
            .v
            .iter()
            .zip(rhs.v)
            .map(|(c1, c2)| c1 * c2)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Mul<&Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: &Vector<N>) -> Self::Output {
        let v = self
            .v
            .iter()
            .zip(rhs.v)
            .map(|(c1, c2)| c1 * c2)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Mul<Float> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: Float) -> Self::Output {
        let v = self
            .v
            .iter()
            .map(|c| c * rhs)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Mul<Float> for &Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: Float) -> Self::Output {
        let v = self
            .v
            .iter()
            .map(|c| c * rhs)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Mul<Vector<N>> for Float {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        let v = rhs
            .v
            .iter()
            .map(|c| c * self)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Mul<&Vector<N>> for Float {
    type Output = Vector<N>;

    fn mul(self, rhs: &Vector<N>) -> Self::Output {
        let v = rhs
            .v
            .iter()
            .map(|c| c * self)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Div<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn div(self, rhs: Vector<N>) -> Self::Output {
        let v = self
            .v
            .iter()
            .zip(rhs.v)
            .map(|(c1, c2)| c1 / c2)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Div<&Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn div(self, rhs: &Vector<N>) -> Self::Output {
        let v = self
            .v
            .iter()
            .zip(rhs.v)
            .map(|(c1, c2)| c1 / c2)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Div<Float> for Vector<N> {
    type Output = Vector<N>;

    fn div(self, rhs: Float) -> Self::Output {
        let recip = rhs.recip();
        let v = self
            .v
            .iter()
            .map(|c| c * recip)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Div<Float> for &Vector<N> {
    type Output = Vector<N>;

    fn div(self, rhs: Float) -> Self::Output {
        let recip = rhs.recip();
        let v = self
            .v
            .iter()
            .map(|c| c * recip)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Neg for Vector<N> {
    type Output = Vector<N>;

    fn neg(self) -> Self::Output {
        let v = self
            .v
            .iter()
            .map(|c| -c)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Neg for &Vector<N> {
    type Output = Vector<N>;

    fn neg(self) -> Self::Output {
        let v = self
            .v
            .iter()
            .map(|c| -c)
            .collect::<Vec<Float>>()
            .try_into()
            .unwrap();
        Vector::<N> { v }
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}

impl<const N: usize> fmt::Display for Vector<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.v)
    }
}

pub fn inner_product<const N: usize>(v1: [Float; N], v2: [Float; N]) -> Float {
    v1.iter().zip(v2).map(|(c1, c2)| c1 * c2).sum()
}

pub type Vector1 = Vector<1>;
pub type Vector2 = Vector<2>;
pub type Vector3 = Vector<3>;
pub type Vector4 = Vector<4>;

#[derive(Copy, Clone, Debug)]
pub struct Bound<const N: usize> {
    pub b: [(Float, Float); N],
}

impl<const N: usize> Bound<N> {
    pub fn new(bound: [(Float, Float); N]) -> Bound<N> {
        let mut b = [(0.0, 0.0); N];
        for (i, v) in bound.iter().enumerate() {
            b[i] = (v.0.min(v.1), v.0.max(v.1));
        }
        Bound::<N> { b }
    }

    pub const fn all() -> Bound<N> {
        let b = [(Float::NEG_INFINITY, Float::INFINITY); N];
        Bound::<N> { b }
    }

    pub fn clamp(&self, vector: &Vector<N>) -> Vector<N> {
        let mut v = [0.0; N];

        for d in 0..N {
            v[d] = (vector[d]).clamp(self.b[d].0, self.b[d].1);
        }

        Vector::<N> { v }
    }

    pub fn contains(&self, v: &Vector<N>) -> bool {
        for i in 0..N {
            if self.b[i].0 > v[i] || self.b[i].1 < v[i] {
                return false;
            }
        }

        true
    }
}

impl<const N: usize> Index<usize> for Bound<N> {
    type Output = (Float, Float);

    fn index(&self, index: usize) -> &Self::Output {
        &self.b[index]
    }
}

impl<const N: usize> IndexMut<usize> for Bound<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.b[index]
    }
}
pub type Interval = Bound<1>;
