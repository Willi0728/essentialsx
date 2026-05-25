use core::ops::{Add, Div, Mul, Sub};

pub struct Complex<R, I> {
    pub re: R,
    pub im: I,
}

impl<R, I> Complex<R, I> {
    #[inline(always)]
    pub fn from_parts(re: R, im: I) -> Self {
        Self { re, im }
    }
}

// we should implement as many traits as possible!
impl<R: Add<J, Output = L>, I: Add<K, Output = M>, J, K, L, M> Add<Complex<J, K>> for Complex<R, I> {
    type Output = Complex<L, M>;
    fn add(self, rhs: Complex<J, K>) -> Self::Output {
        Complex::from_parts(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<R: Sub<J, Output = L>, I: Sub<K, Output = M>, J, K, L, M> Sub<Complex<J, K>> for Complex<R, I> {
    type Output = Complex<L, M>;
    fn sub(self, rhs: Complex<J, K>) -> Self::Output {
        Complex::from_parts(self.re - rhs.re, self.im - rhs.im)
    }
}

impl<R, I, J, K, L, M, N, O> Mul<Complex<J, K>> for Complex<R, I>
where
    R: Mul<J, Output = L> + Clone,
    I: Mul<K, Output = M> + Clone,
    R: Mul<K, Output = N> + Clone,
    I: Mul<J, Output = O> + Clone,
    L: Sub<M>,
    N: Add<O>,
    J: Clone,
    K: Clone
{
    type Output = Complex<<L as Sub<M>>::Output, <N as Add<O>>::Output>;

    fn mul(self, rhs: Complex<J, K>) -> Self::Output {
        // (a + bi)(c + di) = (ac - bd) + (ad + bc)i
        Complex::from_parts(
            self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone(),
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl<A, B, C, D, AC, BD, CC, DD, BC, AD, ACBD, BCAD, CCDD, RO, IO>
    Div<Complex<C, D>> for Complex<A, B>
where
    A: Mul<C, Output = AC>,
    B: Mul<D, Output = BD>,
    C: Mul<C, Output = CC>,
    D: Mul<D, Output = DD>,
    B: Mul<C, Output = BC>,
    A: Mul<D, Output = AD>,
    AC: Add<BD, Output = ACBD>,
    BC: Sub<AD, Output = BCAD>,
    CC: Add<DD, Output = CCDD>,
    ACBD: Div<CCDD, Output = RO>,
    BCAD: Div<CCDD, Output = IO>,
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    CCDD: Clone,
{
    type Output = Complex<RO, IO>;

    /// div does not make sure that this formula is mathematically sound! It only checks for the
    /// minimum set of operators needed to work the formula. If you create a type T where
    /// Add<T, Output=T> actually subtracts it, that's on you. The main reason for this is
    /// heterogeneous multiplication, e.g. f32 + f64 or something crazier.
    fn div(self, rhs: Complex<C, D>) -> Self::Output {
        let (a, b) = (self.re, self.im);
        let (c, d) = (rhs.re, rhs.im);
        let den = c.clone() * c.clone() + d.clone() * d.clone();
        Complex::from_parts(
            (a.clone() * c.clone() + b.clone() * d.clone()) / den.clone(),
            (b * c - a * d) / den,
        )
    }
}

impl<R: Clone, I: Clone> Clone for Complex<R, I> {
    fn clone(&self) -> Self {
        Complex::from_parts(self.re.clone(), self.im.clone())
    }
}

impl<R: Copy, I: Copy> Copy for Complex<R, I> {}