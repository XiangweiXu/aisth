type Float = f64;

pub struct SquareMatrix<const N: usize> {
    m: [[Float; N]; N],
}

impl<const N: usize> SquareMatrix<N> {
    pub fn identity() -> Self {
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

        Self { m }
    }

    pub fn zero() -> Self {
        Self { m: [[0.0; N]; N] }
    }

    pub fn transpose(&self) -> Self {
        let mut m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = self[j][i];
            }
        }
        Self { m }
    }
}

impl<const N: usize> std::ops::Index<usize> for SquareMatrix<N> {
    type Output = [Float; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}


