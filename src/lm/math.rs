type Float = f64;

struct SquareMatrix<const N: u8> {
    mut m: [[Float; N]; N],
}

impl<N> SquareMatrix<N> {
    fn identity() -> Self {
        let m = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                if (i == j) {
                    m[i][j] = 1.0;
                } else {
                    m[i][j] = 0.0;
                }
            }
        }

        Self { m }
    }

    static fn zero() -> Self {
        Self { m: [[0.0; N]; N] }
    }

    fn transpose(&self) {
        for i in 0..N {
            for j in 0..N {
                self
            }
        }        
    }
}

impl<N> std::ops::Index<u8> for SquareMatrix<N> {
    type Output = [Float; N];

    fn index(&self, index: u8) -> &Self::Output {
        self.m[index]
    }
}


