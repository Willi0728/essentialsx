use std::ops::{Add, Deref, DerefMut, Sub};

/*
 * todo:
    [x] left identity
    [x] right identity
    [x] transpose
    [x] scale
    [x] Add (<M, N> + <M, N>)
    [x] Sub (<M, N> - <M, N>)
    [ ] Mul (<M, N> * <N, O>)
    [ ] inverse
    [ ] determinant
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
        let mut j;
        while i < M {
            j = 0;
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