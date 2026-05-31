use core::ops::{Add, Deref, DerefMut, Mul, Sub};

/// A macro for quickly iterating over i and j in const contexts.
/// Compatible with Matrix, to iterate over N and M
#[macro_export]
macro_rules! ij_loop {
    ($iident:ident until $stop1:expr, $jident:ident until $stop2:expr, $body:tt) => {
        let mut $iident = 0;
        while $iident < $stop1 {
            let mut $jident = 0;
            while $jident < $stop2 {
                $body
                $jident += 1;
            }
            $iident += 1;
        }
    };
}

/// A matrix, representing various sizes through the constant generics M and N
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix<const M: usize, const N: usize>(pub [[f64; N]; M]);

impl<const M: usize, const N: usize> Matrix<M, N> {
    /// A zero matrix.
    /// Example:
    /// ```
    /// use essentialsx::math::Matrix;
    ///
    /// let zero = Matrix::<3, 3>::zero();
    /// let something_else = Matrix::<3, 3>::new([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 10.0],
    /// ]);
    /// assert_eq!(zero * something_else, zero);
    /// ```
    #[inline]
    pub const fn zero() -> Self {
        Self([[0.0; N]; M])
    }

    /// The left identity for any size matrix. Note that it does not return Self!
    /// ```
    /// use essentialsx::math::Matrix;
    ///
    /// let left = Matrix::<3, 3>::left_identity();
    /// let something_else = Matrix::<3, 3>::new([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 10.0],
    /// ]);
    /// assert_eq!(left * something_else, something_else);
    /// ```
    #[inline]
    pub const fn left_identity() -> Matrix<M, M> {
        Matrix::<M, M>::identity()
    }

    /// The right identity for any size matrix. Note that it does not return Self!
    /// ```
    /// use essentialsx::math::Matrix;
    ///
    /// let right = Matrix::<3, 3>::right_identity();
    /// let something_else = Matrix::<3, 3>::new([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 10.0],
    /// ]);
    /// assert_eq!(something_else * right, something_else);
    /// ```
    #[inline]
    pub const fn right_identity() -> Matrix<N, N> {
        Matrix::<N, N>::identity()
    }

    /// The transposition of a matrix, $M^{T}$.
    /// ```
    /// use essentialsx::math::Matrix;
    ///
    /// let matrix = Matrix::<3, 3>::new([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 10.0],
    /// ]);
    /// let rotated = Matrix::<3, 3>::new([
    ///     [1.0, 4.0, 7.0],
    ///     [2.0, 5.0, 8.0],
    ///     [3.0, 6.0, 10.0],
    /// ]);
    /// assert_eq!(matrix.transpose(), rotated);
    /// ```
    pub const fn transpose(&self) -> Matrix<N, M> {
        self.transpose_inline()
    }

    /// See [`Matrix::transpose`]
    #[inline(always)]
    pub const fn transpose_inline(&self) -> Matrix<N, M> {
        let mut result = Matrix::<N, M>::zero();
        ij_loop!(i until M, j until N, {
            result.0[j][i] = self.0[i][j];
        });
        result
    }

    /// Multiplies the matrix element-wise.
    /// ```
    /// use essentialsx::math::Matrix;
    ///
    /// let matrix = Matrix::<3, 3>::new([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 10.0],
    /// ]);
    /// let scaled = Matrix::<3, 3>::new([
    ///     [2.0, 4.0, 6.0],
    ///     [8.0, 10.0, 12.0],
    ///     [14.0, 16.0, 20.0],
    /// ]);
    /// assert_eq!(matrix.scale(2.0), scaled);
    /// ```
    pub const fn scale(&self, factor: f64) -> Self {
        let mut result = Matrix::zero();
        ij_loop!(i until M, j until N, {
            result.0[i][j] = self.0[i][j] * factor;
        });
        result
    }

    /// Does the same thing as [`Matrix::scale`], but in-place.
    pub const fn scale_mut(&mut self, factor: f64) {
        ij_loop!(i until M, j until N, {
            self.0[i][j] = self.0[i][j] * factor;
        });
    }

    /// Standard matrix multiplication, inlined into the caller.
    /// See [`Matrix::mul`]
    #[inline(always)]
    pub const fn mul_inline<const O: usize>(&self, rhs: &Matrix<N, O>) -> Matrix<M, O> {
        let mut result = Matrix::<M, O>::zero();
        ij_loop!(i until M, c until N, {
            let self_val = self.0[i][c];
            let mut j = 0;
            while j < O {
                result.0[i][j] += self_val * rhs.0[c][j];
                j += 1;
            }
        });
        result
    }

    /// Resizes a matrix.
    ///
    /// If a dimension is greater, it will extend with zeros.
    /// If a dimension is equal, it will copy exactly.
    /// If a dimension is smaller, it will truncate.
    ///
    /// Internally, resize uses [`Matrix::copy_into`].
    ///
    /// Example:
    /// ```
    /// use essentialsx::math::Matrix;
    ///
    /// let matrix = Matrix::<3, 3>::new([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 10.0],
    /// ]);
    /// let expected_small = Matrix::<2, 2>::new([
    ///     [1.0, 2.0],
    ///     [4.0, 5.0],
    /// ]);
    /// let expected_larger = Matrix::<4, 4>::new([
    ///     [1.0, 2.0, 3.0, 0.0],
    ///     [4.0, 5.0, 6.0, 0.0],
    ///     [7.0, 8.0, 10.0, 0.0],
    ///     [0.0, 0.0, 0.0, 0.0]
    /// ]);
    /// assert_eq!(matrix.resize::<2, 2>(), expected_small);
    /// assert_eq!(matrix.resize::<3, 3>(), matrix);
    /// assert_eq!(matrix.resize::<4, 4>(), expected_larger);
    /// ```
    pub const fn resize<const O: usize, const P: usize>(&self) -> Matrix<O, P> {
        let mut data = [[0.0; P]; O];
        if const { (O == M) & (P == N) } {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    self.0.as_ptr() as *const f64,
                    data.as_mut_ptr() as *mut f64,
                    M * N,
                );
            }
            return Matrix(data)
        }
        self.copy_into(&mut data);
        Matrix(data)
    }

    /// Self-explanatory - this function copies the inner contents of the Matrix into another
    /// nested array struction that does not have to be the same size. If the resulting size is the
    /// same, it directly copies. If it is bigger, it will leave the elements with a larger index
    /// untouched. If smaller, it will truncate.
    pub const fn copy_into<const O: usize, const P: usize>(&self, data: &mut [[f64; P]; O]) {
        let min_om = const { (O + M - O.abs_diff(M)) / 2 };
        let min_pn = const { (P + N - P.abs_diff(N)) / 2 };

        ij_loop!(i until min_om, j until min_pn, {
            data[i][j] = self.0[i][j];
        });
    }

    /// Shrinks the matrix by one element on the right and bottom, discarding the outer elements.
    /// This is the non-unstable version, meaning you have to pass in the original matrix size as
    /// generic arguments. [`Matrix::resize`] is preferred for this, though.
    ///
    /// Example:
    /// ```
    /// use essentialsx::math::Matrix;
    ///
    /// let matrix = Matrix::<3, 3>::new([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 10.0],
    /// ]);
    /// let expected = Matrix::<2, 2>::new([
    ///     [1.0, 2.0],
    ///     [4.0, 5.0]
    /// ]);
    ///
    /// assert_eq!(Matrix::shrink_one(&matrix), expected)
    /// ```
    #[cfg(not(feature = "unstable"))]
    #[inline]
    pub const fn shrink_one<const FROM_M: usize, const FROM_N: usize>(
        from: &Matrix<FROM_M, FROM_N>,
    ) -> Self {
        const {
            assert!(FROM_M == M + 1);
            assert!(FROM_N == N + 1);
        }
        let mut data = [[0.0; N]; M];
        ij_loop!(r until M, c until N, {
            data[r][c] = from.0[r][c]
        });
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

    const fn pop_row_impl<const O: usize>(&self, row: usize) -> Matrix<O, N> {
        assert!(row < M);
        #[cfg(not(feature = "unstable"))]
        const {
            assert!(O + 1 == M);
        }

        let mut data = [[0.0; N]; O];

        let mut src = 0;
        let mut dst = 0;

        while src < M {
            if src != row {
                data[dst] = self.0[src];
                dst += 1;
            }
            src += 1;
        }

        Matrix(data)
    }

    #[cfg(feature = "unstable")]
    #[inline(always)]
    pub const fn pop_row(&self, row: usize) -> Matrix<{ M - 1 }, N>
    where
        [(); M - 1]: Sized,
    {
        self.pop_row_impl::<{ M - 1 }>(row)
    }

    #[cfg(not(feature = "unstable"))]
    #[inline(always)]
    pub const fn pop_row<const O: usize>(&self, row: usize) -> Matrix<O, N> {
        self.pop_row_impl::<O>(row)
    }
    pub const fn pop_col_impl<const O: usize>(&self, col: usize) -> Matrix<M, O> {
        assert!(col < N);
        #[cfg(not(feature = "unstable"))]
        const {
            assert!(O + 1 == N);
        }
        let mut result = [[0.0; O]; M];
        let mut r = 0;

        while r < M {
            let mut c = 0;
            let mut dest_c = 0;
            while c < N {
                if c != col {
                    result[r][dest_c] = self.0[r][c];
                    dest_c += 1;
                }
                c += 1;
            }
            r += 1;
        }
        Matrix(result)
    }

    #[cfg(feature = "unstable")]
    #[inline(always)]
    pub const fn pop_col(&self, col: usize) -> Matrix<M, { N - 1 }>
    where
        [(); N - 1]: Sized,
    {
        self.pop_col_impl::<{ N - 1 }>(col)
    }

    #[cfg(not(feature = "unstable"))]
    #[inline(always)]
    pub const fn pop_col<const O: usize>(&self, col: usize) -> Matrix<M, O> {
        self.pop_col_impl::<O>(col)
    }

    pub const fn scalar_mul_inplace(&mut self, scalar: f64) {
        ij_loop!(i until M, j until N, {
            self.0[i][j] = self.0[i][j] * scalar;
        });
    }

    pub const fn scalar_mul(&self, scalar: f64) -> Self {
        let mut copy = Matrix(self.0);
        copy.scalar_mul_inplace(scalar);
        copy
    }

    pub const fn is_square(&self) -> bool {
        const { M == N }
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

    pub const fn try_inverse_impl<const O: usize>(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < 1e-9 { return None; }
        let inv_det = 1.0 / det;
        let mut copy = Matrix(self.0);
        //minors
        ij_loop!(i until M, j until M, {
            let removed = self.pop_row_impl::<O>(i).pop_col_impl::<O>(j);
            copy.0[i][j] = removed.determinant();
        });
        //cofactors
        ij_loop!(i until M, j until M, {
            copy.0[i][j] *= if (i + j) % 2 == 0 { 1.0 } else { -1.0 }
        });
        //adjugate
        copy = copy.transpose();
        //inverse
        copy.scalar_mul_inplace(inv_det);
        Some(copy)
    }

    const fn matrix_elimination_impl<const N: usize>(&self) -> Matrix<N, N> {
        let mut previous = [[1.0; M]; M];
        let mut current = self.0;
        let mut size = M;

        while size > N {
            let mut next = Matrix::zero().0;
            let mut r = 0;
            while r + 1 < size {
                let mut c = 0;
                while c + 1 < size {
                    next[r][c] = (current[r][c] * current[r + 1][c + 1]
                        - current[r][c + 1] * current[r + 1][c])
                        / previous[r + 1][c + 1];
                    c += 1;
                }
                r += 1;
            }
            previous = current;
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
    /// And does not handle zero pivots -- will return inf, -inf, or NaN
    ///
    /// How not to use:
    /// ```compile_fail
    /// use essentialsx::math::Matrix;
    ///
    /// let matrix = Matrix::<2, 2>::identity();
    /// let _ = matrix.matrix_elimination::<3>(); // FAIL: 3 > 2
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

    pub const fn determinant(&self) -> f64 {
        let (swaps, upper_triangular) = self.to_upper_triangular();
        let mut diagonal = 1.0;
        let mut i = 0;
        while i < M {
            diagonal *= upper_triangular.0[i][i];
            i += 1;
        }
        if swaps % 2 != 0 { -diagonal } else { diagonal }
    }

    pub const fn make_upper_triangular(&mut self) -> u32 {
        let mut swaps = 0;
        let mut i = 0;
        while i < M {
            let mut pivot_row = i;
            let mut max_val = self.0[i][i];
            if max_val < 0.0 {
                max_val = -max_val;
            }
            let mut r = i + 1;
            while r < M {
                let mut val = self.0[r][i];
                if val < 0.0 {
                    val = -val;
                }
                if val > max_val {
                    max_val = val;
                    pivot_row = r;
                }
                r += 1;
            }
            if pivot_row != i {
                let mut c = 0;
                while c < M {
                    let tmp = self.0[i][c];
                    self.0[i][c] = self.0[pivot_row][c];
                    self.0[pivot_row][c] = tmp;
                    c += 1;
                }
                swaps += 1;
            }
            if self.0[i][i] == 0.0 {
                i += 1;
                continue;
            }
            let mut r = i + 1;
            while r < M {
                let factor = self.0[r][i] / self.0[i][i];
                let mut c = i;
                while c < M {
                    self.0[r][c] -= factor * self.0[i][c];
                    c += 1;
                }
                r += 1;
            }
            i += 1;
        }
        swaps
    }
    pub const fn to_upper_triangular(&self) -> (u32, Self) {
        let mut copied = Matrix(self.0);
        let flips = copied.make_upper_triangular();
        (flips, copied)
    }

    pub const fn is_symmetric(&self) -> bool {
        ij_loop!(i until M, j until M, {
            if self.0[i][j] != self.0[j][i] {
                return false;
            }
        });
        true
    }

    pub const fn is_diagonal(&self) -> bool {
        ij_loop!(i until M, j until M, {
            if i == j { j += 1; continue; }
            if self.0[i][j] != 0.0 {
                return false;
            }
        });
        true
    }
}

impl Matrix<2, 2> {
    #[inline(always)]
    pub const fn determinant_2x2(&self) -> f64 {
        self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[1][0]
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

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;

    #[test]
    fn matrix_elimination_produces_adjacent_2x2_determinants() {
        let matrix = Matrix::<3, 3>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 10.0]]);

        let eliminated = matrix.matrix_elimination::<2>();
        assert_eq!(*eliminated, [[-3.0, -3.0], [-3.0, 2.0]]);
    }

    #[test]
    fn matrix_elimination_produces_correct_determinant() {
        let matrix = Matrix::<3, 3>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 10.0]]);

        let eliminated = matrix.matrix_elimination::<1>();
        assert_eq!(eliminated[0][0], -3.0);
    }
    #[test]
    fn matrix_elimination_to_1x1_matches_2x2_determinant() {
        let matrix = Matrix::<2, 2>::new([[3.0, 8.0], [4.0, 6.0]]);

        let eliminated = matrix.matrix_elimination::<1>();

        assert_eq!(eliminated.0, [[-14.0]]);
        assert_eq!(matrix.determinant_2x2(), -14.0);
    }

    #[test]
    fn determinant_handles_zero_by_zero() {
        let matrix = Matrix::new([]);
        assert_eq!(matrix.determinant(), 1.0);
    }

    #[test]
    fn determinant_handles_one_by_one() {
        let matrix = Matrix::new([[7.5]]);
        assert_eq!(matrix.determinant(), 7.5);
    }

    #[test]
    fn determinant_handles_two_by_two() {
        let matrix = Matrix::new([[3.0, 8.0], [4.0, 6.0]]);
        assert_eq!(matrix.determinant(), -14.0);
    }

    #[test]
    fn determinant_handles_three_by_three() {
        let matrix = Matrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 10.0]]);
        assert_relative_eq!(matrix.determinant().round(), -3.0);
    }
}