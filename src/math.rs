use core::ops::{Add, Deref, DerefMut, Mul, Sub};

/*
 * todo:
    [x] left identity
    [x] right identity
    [x] transpose
    [x] scale
    [x] Add (<M, N> + <M, N>)
    [x] Sub (<M, N> - <M, N>)
    [x] Mul (<M, N> * <N, O>)
    [ ] inverse
    [x] determinant
    [ ] trace
    [ ] is_* functions:
        [ ] square
        [ ] symmetric
        [ ] diagonal
 */

#[derive(Debug, Clone)]
pub struct Matrix<const M: usize, const N: usize> (pub [[f64; N]; M]);

impl<const M: usize, const N: usize> Matrix<M, N> {
    #[inline]
    pub const fn zero() -> Self {
        Self([[0.0; N]; M])
    }

    #[inline]
    pub const fn left_identity() -> Matrix<M, M> {
        Matrix::<M, M>::identity()
    }

    #[inline]
    pub const fn right_identity() -> Matrix<N, N> {
        Matrix::<N, N>::identity()
    }

    pub const fn transpose(&self) -> Matrix<N, M> {
        let mut result = Matrix::<N, M>::zero();
        let mut i = 0;
        while i < M {
            let mut j = 0;
            while j < N {
                result.0[j][i] = self.0[i][j];
                j += 1;
            }
            i += 1;
        }
        result
    }

    pub const fn scale(&self, factor: f64) -> Matrix<M, N> {
        let mut result = Matrix::<M, N>::zero();
        let mut i = 0;
        let mut j;
        while i < M {
            j = 0;
            while j < N {
                result.0[i][j] = self.0[i][j] * factor;
                j += 1;
            }
            i += 1;
        }
        result
    }

    pub const fn scale_mut(&mut self, factor: f64) {
        let mut i = 0;
        let mut j;
        while i < M {
            j = 0;
            while j < N {
                self.0[i][j] = self.0[i][j] * factor;
                j += 1;
            }
            i += 1;
        }
    }

    #[inline(always)]
    pub const fn mul_inline<const O: usize>(&self, rhs: &Matrix<N, O>) -> Matrix<M, O> {
        let mut result = Matrix::<M, O>::zero();
        let mut i = 0;
        while i < M {
            let mut c = 0;
            while c < N {
                let self_val = self.0[i][c];
                let mut j = 0;
                while j < O {
                    result.0[i][j] += self_val * rhs.0[c][j];
                    j += 1;
                }
                c += 1;
            }
            i += 1;
        }
        result
    }

    #[cfg(feature = "unstable")]
    pub const fn shrink<const O: usize, const P: usize>(&self) -> Matrix<O, P>
    where
        [(); M - O]: Sized,
        [(); N - P]: Sized,
    {
        let mut data = [[0.0; P]; O];
        self.copy_into(&mut data);
        Matrix(data)
    }

    #[cfg(not(feature = "unstable"))]
    pub const fn shrink<const O: usize, const P: usize>(&self) -> Matrix<O, P> {
        const {
            assert!(O <= M);
            assert!(P <= N);
        }
        let mut data = [[0.0; P]; O];
        self.copy_into(&mut data);
        Matrix(data)
    }

    pub const fn copy_into<const O: usize, const P: usize>(&self, data: &mut [[f64; P]; O]) {
        let mut r = 0;
        while r < O {
            let mut c = 0;
            while c < P {
                data[r][c] = self.0[r][c];
                c += 1;
            }
            r += 1;
        }
    }

    #[cfg(not(feature = "unstable"))]
    #[inline]
    pub const fn shrink_one<const FROM_M: usize, const FROM_N: usize>(
        from: &Matrix<FROM_M, FROM_N>
    ) -> Self {
        const {
            assert!(FROM_M == M + 1);
            assert!(FROM_N == N + 1);
        }
        let mut data = [[0.0; N]; M];
        let mut r = 0;
        while r < M {
            let mut c = 0;
            while c < N {
                data[r][c] = from.0[r][c];
                c += 1;
            }
            r += 1;
        }
        Self(data)
    }

    #[cfg(feature = "unstable")]
    #[inline]
    pub const fn shrink_one(&self) -> Matrix<{ M - 1 }, { N - 1 }>
    where
        [(); M - 1]: Sized,
        [(); N - 1]: Sized,
    {
        let mut data = [[0.0; { N - 1 }]; { M - 1 }];
        let mut r = 0;
        while r < M - 1 {
            let mut c = 0;
            while c < N - 1 {
                data[r][c] = self.0[r][c];
                c += 1;
            }
            r += 1;
        }
        Matrix(data)
    }
    pub const fn new(data: [[f64; N]; M]) -> Self {
        Self(data)
    }
}

impl<const M: usize> Matrix<M, M> {
    #[inline]
    pub const fn identity() -> Self {
        let mut result = Self::zero();
        let mut i = 0;
        while i < M {
            result.0[i][i] = 1.0;
            i += 1;
        }
        result
    }

    // pub const fn invert(&self) -> Self {
    //     todo!("On Hold: requires Determinant");
    // }

    const fn matrix_elimination_impl<const N: usize>(&self) -> Matrix<N, N> {
        let mut current = self.0;
        let mut size = M;

        while size > N {
            let mut next = [[0.0; M]; M];
            let mut r = 0;
            while r + 1 < size {
                let mut c = 0;
                while c + 1 < size {
                    next[r][c] = current[r][c] * current[r + 1][c + 1]
                        - current[r][c + 1] * current[r + 1][c];
                    c += 1;
                }
                r += 1;
            }
            current = next;
            size -= 1;
        }

        let mut result = [[0.0; N]; N];
        let mut r = 0;
        while r < N {
            let mut c = 0;
            while c < N {
                result[r][c] = current[r][c];
                c += 1;
            }
            r += 1;
        }

        Matrix(result)
    }

    /// Helper function for Dodgson Condensation. Will repeat until the matrix is NxN
    ///
    /// ```compile_fail
    /// use essentialsx::math::Matrix;
    ///
    /// let matrix = Matrix::<2, 2>::identity();
    /// let _ = matrix.matrix_elimination::<3>();
    /// ```
    #[cfg(not(feature = "unstable"))]
    pub const fn matrix_elimination<const N: usize>(&self) -> Matrix<N, N> {
        const {
            assert!(N <= M);
        }
        self.matrix_elimination_impl::<N>()
    }

    /// Helper function for Dodgson Condensation. Will repeat until the matrix is NxN
    ///
    /// ```compile_fail
    /// use essentialsx::math::Matrix;
    ///
    /// let matrix = Matrix::<2, 2>::identity();
    /// let _ = matrix.matrix_elimination::<3>();
    /// ```
    #[cfg(feature = "unstable")]
    pub const fn matrix_elimination<const N: usize>(&self) -> Matrix<N, N>
    where
        [(); M - N]: Sized,
    {
        self.matrix_elimination_impl::<N>()
    }

    pub const fn trace(&self) -> f64 {
        let mut i = 0;
        let mut total = 0.0;
        while i < M {
            total += self.0[i][i];
            i += 1;
        }
        total
    }
}

impl Matrix<2, 2> {
    pub const fn determinant_2x2(&self) -> f64 {
        self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[1][0]
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn matrix_elimination_produces_adjacent_2x2_determinants() {
        let matrix = Matrix::<3, 3>::new([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 10.0],
        ]);

        let eliminated = matrix.matrix_elimination::<2>();

        assert_eq!(eliminated.0, [[-3.0, -3.0], [-3.0, 2.0]]);
    }

    #[test]
    fn matrix_elimination_to_1x1_matches_2x2_determinant() {
        let matrix = Matrix::<2, 2>::new([
            [3.0, 8.0],
            [4.0, 6.0],
        ]);

        let eliminated = matrix.matrix_elimination::<1>();

        assert_eq!(eliminated.0, [[-14.0]]);
        assert_eq!(matrix.determinant_2x2(), -14.0);
    }
}

impl<const M: usize, const N: usize> Deref for Matrix<M, N> {
    type Target = [[f64; N]; M];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const M: usize, const N: usize> DerefMut for Matrix<M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! per_op {
    ($trait_name:ident, $fn_name:ident, $op:tt) => (
        impl<const M: usize, const N: usize> $trait_name for Matrix<M, N> {
            type Output = Self;

            fn $fn_name(self, rhs: Self) -> Self {
                let mut result = Self::zero();
                for (i, v) in self.0.iter().enumerate() {
                    for (j, &v) in v.iter().enumerate() {
                        result[i][j] = v $op rhs[i][j];
                    }
                }
                result
            }
        }
    )
}

per_op!(Add, add, +);
per_op!(Sub, sub, -);

impl<const M: usize, const N: usize, const O: usize> Mul<Matrix<N, O>> for Matrix<M, N> {
    type Output = Matrix<M, O>;

    //standard is O(n^3)
    fn mul(self, rhs: Matrix<N, O>) -> Self::Output {
        let mut result = Self::Output::zero();
        for i in 0..M {
            for j in 0..O {
                for c in 0..N {
                    result[i][j] += self[i][c] * rhs.0[c][j];
                }
            }
        }
        result
    }
}
