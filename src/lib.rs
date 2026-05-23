#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "unstable", allow(incomplete_features))]
#![cfg_attr(feature = "unstable", feature(generic_const_exprs))]
#[cfg(feature = "io")]
pub mod io;
#[cfg(feature = "math")]
pub mod math;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
