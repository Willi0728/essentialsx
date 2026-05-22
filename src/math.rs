use std::ops::{Deref, DerefMut};

/*
 * todo:
    [x] left identity
    [x] right identity
    [ ] transpose
    [ ] scale
    [ ] Add (<M, N> + <M, N>)
    [ ] Sub (<M, N> - <M, N>)
    [ ] Mul (<M, N> * <N, O>)
    [ ] inverse
    [ ] determinant
    [ ] trace
    [ ] is_* functions:
        [ ] square
        [ ] symmetric
        [ ] diagonal
 */

pub struct Matrix<const M: usize, const N: usize> (pub [[f64; N]; M]);

impl<const M: usize, const N: usize> Matrix<M, N> {
    #[inline]
    const fn zero() -> Self {
        Self([[0.0; N]; M])
    }

    #[inline]
    const fn left_identity() -> Matrix<M, M> {
        Matrix::<M, M>::identity()
    }

    #[inline]
    const fn right_identity() -> Matrix<N, N> {
        Matrix::<N, N>::identity()
    }
}

impl<const M: usize> Matrix<M, M> {
    #[inline]
    const fn identity() -> Self {
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