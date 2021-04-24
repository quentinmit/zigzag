//! A crate which exposes two quasi-extension traits that extend signed integers
//! with the [`ZigZagEncode`] trait and unsigned integers with the
//! [`ZigZagDecode`] trait.
//!
//! # Zigzag Encoding
//!
//! Zigzag encoding takes a signed integer and encodes it as an unsigned
//! integer. It does so by counting up, starting at zero, alternating
//! between representing a positive number and a negative number.
//!
//! To encode any signed integer, `x`, with a representation length—in bits,
//! `n`, the formula is as follows:
//!
//! ```txt
//! (x >> n - 1) ^ (x << 1)
//! ```
//!
//! *Note*: The `^` operator *is not* exponentiation; rather, it is bitwise
//! XOR.
//!
//! For instance, the simplified formula to encode a signed *32-bit* integer
//! of value `x` would be as follows:
//!
//! ```txt
//! (x >> 31) ^ (x << 1)
//! ```
//!
//! # Zigzag Decoding
//!
//! To convert a zigzag-encoded unsigned integer, `x`, to its decoded signed
//! counterpart, the formula is as follows:
//!
//! ```txt
//! (x >>> 1) ^ -(x & 1)
//! ```
//!
//! *Note*: The `>>>` operator is to represent a logical right shift as opposed
//! to an arithmetic right shift. In Rust, unsigned integer types implement the
//! right shift operator as logical instead of arithmetic. Therefore, the
//! formula in Rust is simplified as `(x >> 1) ^ -(x & 1)`.
//!
//! # Examples
//!
//! Encoding a signed integer:
//!
//! ```
//! use zigzag::ZigZagEncode;
//!
//! assert_eq!(0i8.zigzag_encode(), 0u8);
//! assert_eq!((-1i8).zigzag_encode(), 1u8);
//! assert_eq!(1i8.zigzag_encode(), 2u8);
//!
//! assert_eq!(i8::MIN.zigzag_encode(), u8::MAX);
//! assert_eq!(i16::MIN.zigzag_encode(), u16::MAX);
//! assert_eq!(i32::MIN.zigzag_encode(), u32::MAX);
//! assert_eq!(i64::MIN.zigzag_encode(), u64::MAX);
//! assert_eq!(i128::MIN.zigzag_encode(), u128::MAX);
//!
//! assert_eq!(isize::MIN.zigzag_encode(), usize::MAX);
//! ```
//!
//! Decoding an unsigned integer:
//!
//! ```
//! use zigzag::ZigZagDecode;
//!
//! assert_eq!(0u8.zigzag_decode(), 0i8);
//! assert_eq!(1u8.zigzag_decode(), -1i8);
//! assert_eq!(2u8.zigzag_decode(), 1i8);
//!
//! assert_eq!(u8::MAX.zigzag_decode(), i8::MIN);
//! assert_eq!(u16::MAX.zigzag_decode(), i16::MIN);
//! assert_eq!(u32::MAX.zigzag_decode(), i32::MIN);
//! assert_eq!(u64::MAX.zigzag_decode(), i64::MIN);
//! assert_eq!(u128::MAX.zigzag_decode(), i128::MIN);
//!
//! assert_eq!(usize::MAX.zigzag_decode(), isize::MIN);
//! ```

use std::mem::size_of;

const BITS_PER_BYTE: usize = 8;

/// A trait intended to extend signed integer types with the ability to get
/// their unsigned representation as derived by zigzag encoding.
///
/// This trait does so by implementing the [`zigzag_encode`] method.
///
/// [`zigzag_encode`]: ZigZagEncode::zigzag_encode
pub trait ZigZagEncode<U> {
    /// Decodes `self` into its unsigned counterpart by using zigzag encoding.
    ///
    /// For more information on zigzag encoding, see its section in the
    /// [crate documentation](crate).
    ///
    /// # Examples
    ///
    /// ```
    /// use zigzag::ZigZagEncode;
    ///
    /// assert_eq!(0i8.zigzag_encode(), 0u8);
    /// assert_eq!((-1i8).zigzag_encode(), 1u8);
    /// assert_eq!(1i8.zigzag_encode(), 2u8);
    /// ```
    fn zigzag_encode(self) -> U;
}

macro_rules! impl_encode {
    ($signed:ty, $unsigned:ty) => {
        impl ZigZagEncode<$unsigned> for $signed {
            #[inline]
            fn zigzag_encode(self) -> $unsigned {
                const TYPE_BITS: usize = size_of::<$unsigned>() * BITS_PER_BYTE;
                (self >> TYPE_BITS - 1) as $unsigned ^ (self << 1) as $unsigned
            }
        }
    };
}

impl_encode!(i8, u8);
impl_encode!(i16, u16);
impl_encode!(i32, u32);
impl_encode!(i64, u64);
impl_encode!(i128, u128);
impl_encode!(isize, usize);

/// A trait intended to extend unsigned integer types with the ability to get
/// their signed representation as derived by zigzag decoding.
///
/// This trait does so by implementing the [`zigzag_decode`] method.
///
/// [`zigzag_decode`]: ZigZagDecode::zigzag_decode
pub trait ZigZagDecode<S> {
    /// Decodes `self` into its signed counterpart by using zigzag decoding.
    ///
    /// For more information on zigzag decoding, see its section in the
    /// [crate documentation](crate).
    ///
    /// # Examples
    ///
    /// ```
    /// use zigzag::ZigZagDecode;
    ///
    /// assert_eq!(0u8.zigzag_decode(), 0i8);
    /// assert_eq!(1u8.zigzag_decode(), -1i8);
    /// assert_eq!(2u8.zigzag_decode(), 1i8);
    /// ```
    fn zigzag_decode(self) -> S;
}

macro_rules! impl_decode {
    ($unsigned:ty, $signed:ty) => {
        impl ZigZagDecode<$signed> for $unsigned {
            #[inline]
            fn zigzag_decode(self) -> $signed {
                (self >> 1) as $signed ^ -((self & 1) as $signed)
            }
        }
    };
}

impl_decode!(u8, i8);
impl_decode!(u16, i16);
impl_decode!(u32, i32);
impl_decode!(u64, i64);
impl_decode!(u128, i128);
impl_decode!(usize, isize);
