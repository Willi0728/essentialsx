mod matrix;
pub use matrix::Matrix;

mod complex;
pub use complex::Complex;
// add complex numbers a+bi generic over A and B
// custom number traits: implements Add, Sub, Mul, Div, Copy, Clone, Default, Hash, PartialEq, Eq,
// PartialOrd, Ord, Send, Sync, Sized, Any, Rem, AddAssign, SubAssign, MulAssign, DivAssign,
// RemAssign, Neg, Not, BitAnd, BitOr, BitXor, Shl, Shr, BitAndAssign, BitOrAssign, BitXorAssign,
// ShlAssign, ShrAssign, Display, Debug, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
// FromStr, Product, Sum, Step

mod nibble {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct u4(u8);
    macro_rules! impl_op_binary {
        ($($trait:ident, $fn_name:ident$(,)?)*) => { $(
            impl ::core::ops::$trait for u4 {
                type Output = u4;
                #[inline(always)]
                fn $fn_name(self, rhs: Self) -> Self::Output {
                    u4(self.0.$fn_name(rhs.0) %16)
                }
            }
        )* }
    }
    impl_op_binary! {
        Add, add,
        Sub, sub,
        Mul, mul,
        Div, div,
        Rem, rem,
        BitAnd, bitand,
        BitOr, bitor,
        BitXor, bitxor,
    }
    macro_rules! impl_op_unary {
        ($($trait:ident, $fn_name:ident $(,)?)*) => { $(
            impl ::core::ops::$trait for u4 {
                type Output = u4;
                #[inline(always)]
                fn $fn_name(self) -> Self::Output {
                    u4(self.0.$fn_name() %16)
                }
            }
        )* };
    }
    impl_op_unary! {
        Not, not,
    }
    macro_rules! impl_op_assign_binary {
        ($($trait:ident, $fn_name:ident $(,)?)*) => { $(
            impl ::core::ops::$trait for u4 {
                #[inline(always)]
                fn $fn_name(&mut self, rhs: Self) {
                    self.0.$fn_name(rhs.0);
                    self.0 %= 16
                }
            }
        )* };
    }

    impl_op_assign_binary! {
        AddAssign, add_assign,
        SubAssign, sub_assign,
        MulAssign, mul_assign,
        DivAssign, div_assign,
        RemAssign, rem_assign,
        BitAndAssign, bitand_assign,
        BitOrAssign, bitor_assign,
        BitXorAssign, bitxor_assign,
    }
}