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