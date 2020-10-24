pub trait ReverseEndian {
    /// Reverse the endianness
    fn reverse_endian(self) -> Self;
}

macro_rules! reverse_endian_impl {
    ($ty: ty, $ty_mod: ident) => {
        impl ReverseEndian for $ty {
            fn reverse_endian(self) -> Self {
                $ty_mod::from_be(self.to_le())
            }
        }
    };
}

reverse_endian_impl!(usize, usize);
reverse_endian_impl!(u16, u16);
reverse_endian_impl!(u32, u32);
reverse_endian_impl!(u64, u64);
reverse_endian_impl!(u128, u128);
reverse_endian_impl!(isize, isize);
reverse_endian_impl!(i16, i16);
reverse_endian_impl!(i32, i32);
reverse_endian_impl!(i64, i64);
reverse_endian_impl!(i128, i128);

// Re-export #[derive(ReverseEndian)].
//
// The reason re-exporting is not enabled by default is that disabling it would
// be annoying for crates that provide handwritten impls.
#[cfg(feature = "reverse_endian_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate reverse_endian_derive;
#[cfg(feature = "reverse_endian_derive")]
#[doc(hidden)]
pub use reverse_endian_derive::*;

#[cfg(test)]
mod tests {
    use super::ReverseEndian;

    #[test]
    fn test_u32() {
        let original: u32 = 0xd4c3b2a1;
        let reversed: u32 = 0xa1b2c3d4;
        assert_eq!(reversed, original.reverse_endian());
    }
}
