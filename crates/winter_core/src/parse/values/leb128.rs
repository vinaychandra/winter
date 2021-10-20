//! https://github.com/milkey-mouse/nom-leb128

use nom::{
    error::{make_error, ContextError, ErrorKind, ParseError},
    IResult, InputIter, InputLength, Needed, Slice,
};
use num_traits::{PrimInt, Signed, WrappingNeg};
use std::{
    mem::size_of,
    num::NonZeroUsize,
    ops::{BitOrAssign, RangeFrom},
};

const NEED_ONE: Needed = Needed::Size(NonZeroUsize::new(1).unwrap());

/// Maximum LEB128-encoded size of an integer type
const fn leb128_size<T>() -> usize {
    let bits = size_of::<T>() * 8;
    (bits + 6) / 7 // equivalent to ceil(bits/7) w/o floats
}

macro_rules! impl_generic_leb128 {
    ($fn_name:ident, $int_ty:ident, $post:tt, $int_name:expr) => {
        #[doc = "Recognizes an LEB128-encoded number that fits in a `"]
        #[doc=$int_name]
        #[doc = "`."]
        #[inline]
        pub fn $fn_name<I, E>(input: I) -> IResult<I, $int_ty, E>
        where
            I: Clone + Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
            E: ParseError<I> + ContextError<I>,
        {
            let mut res = 0;
            let mut shift = 0;

            for (pos, byte) in input.iter_indices() {
                if (byte & 0x80) == 0 {
                    res |= (byte as $int_ty) << shift;
                    $post(&mut res, shift, byte);
                    return Ok((input.slice(pos + 1..), res));
                } else if pos == leb128_size::<$int_ty>() - 1 {
                    return Err(nom::Err::Error(E::add_context(
                        input.clone(),
                        concat!("LEB128 integer is too big to fit in ", $int_name),
                        make_error(input, ErrorKind::TooLarge),
                    )));
                } else {
                    res |= ((byte & 0x7F) as $int_ty) << shift;
                }
                shift += 7;
            }

            Err(nom::Err::Incomplete(NEED_ONE))
        }
    };
    ($fn_name:ident, $int_ty:ident, $post:tt) => {
        impl_generic_leb128!($fn_name, $int_ty, $post, stringify!($int_ty));
    };
}

#[inline]
fn ignore<T: BitOrAssign + PrimInt + WrappingNeg>(_res: &mut T, _shift: usize, _byte: u8) {}

macro_rules! impl_unsigned_leb128 {
    ($fn_name:ident, $int_ty:ident) => {
        impl_generic_leb128!($fn_name, $int_ty, ignore);
    };
}

impl_unsigned_leb128!(leb128_u8, u8);
impl_unsigned_leb128!(leb128_u16, u16);
impl_unsigned_leb128!(leb128_u32, u32);
impl_unsigned_leb128!(leb128_u64, u64);
impl_unsigned_leb128!(leb128_u128, u128);
impl_unsigned_leb128!(leb128_usize, usize);

#[inline]
fn sign_extend<T: BitOrAssign + PrimInt + Signed + WrappingNeg>(
    res: &mut T,
    shift: usize,
    byte: u8,
) {
    // leb128_generic skips the last shift update for efficiency on unsigned ints

    if (shift < size_of::<T>() * 8 - 7) && ((byte & 0x40) != 0) {
        // sign extend
        *res |= (T::one() << (shift + 7)).wrapping_neg()
    }
}

macro_rules! impl_signed_leb128 {
    ($fn_name:ident, $int_ty:ident) => {
        impl_generic_leb128!($fn_name, $int_ty, sign_extend);
    };
}

impl_signed_leb128!(leb128_i8, i8);
impl_signed_leb128!(leb128_i16, i16);
impl_signed_leb128!(leb128_i32, i32);
impl_signed_leb128!(leb128_i64, i64);
impl_signed_leb128!(leb128_i128, i128);
impl_signed_leb128!(leb128_isize, isize);

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use super::*;

    #[test]
    fn test_leb128() {
        assert_eq!(leb128_size::<u8>(), 2);
        type ResType<'a> = Result<(&'a [u8], u8), nom::Err<VerboseError<&'a [u8]>>>;

        let value: ResType = leb128_u8(&[0x03]);
        assert_eq!(value, Ok((&[][..], 3)));
        let value: ResType = leb128_u8(&[0x83, 0x00]);
        assert_eq!(value, Ok((&[][..], 3)), "Should consume upto n/7 bytes");
        let value: ResType = leb128_u8(&[0x83, 0x00, 0x00]);
        assert_eq!(value, Ok((&[0x00][..], 3)));
        let value: ResType = leb128_u8(&[0x83, 0x80, 0x00]);
        assert!(value.is_err(), "Should fail on too large input");

        assert_eq!(leb128_size::<u16>(), 3);
        type ResType2<'a> = Result<(&'a [u8], u16), nom::Err<VerboseError<&'a [u8]>>>;

        let value: ResType2 = leb128_u16(&[0x03]);
        assert_eq!(value, Ok((&[][..], 3)));
        let value: ResType2 = leb128_u16(&[0x83, 0x00]);
        assert_eq!(value, Ok((&[][..], 3)));
        let value: ResType2 = leb128_u16(&[0x83, 0x80, 0x00]);
        assert_eq!(value, Ok((&[][..], 3)));
        let value: ResType2 = leb128_u16(&[0x83, 0x80, 0x00, 0x00]);
        assert_eq!(value, Ok((&[0x00][..], 3)));
        let value: ResType2 = leb128_u16(&[0x83, 0x80, 0x80, 0x00, 0x00]);
        assert!(value.is_err(), "Should fail on too large input");

        assert_eq!(leb128_size::<i16>(), 3);
        type ResType3<'a> = Result<(&'a [u8], i16), nom::Err<VerboseError<&'a [u8]>>>;

        let value: ResType3 = leb128_i16(&[0x03]);
        assert_eq!(value, Ok((&[][..], 3)));
        let value: ResType3 = leb128_i16(&[0x83, 0x00]);
        assert_eq!(value, Ok((&[][..], 3)));
        let value: ResType3 = leb128_i16(&[0x83, 0x80, 0x00]);
        assert_eq!(value, Ok((&[][..], 3)));
        let value: ResType3 = leb128_i16(&[0x83, 0x80, 0x00, 0x00]);
        assert_eq!(value, Ok((&[0x00][..], 3)));
        let value: ResType3 = leb128_i16(&[0x83, 0x80, 0x80, 0x00, 0x00]);
        assert!(value.is_err(), "Should fail on too large input");

        let value: ResType3 = leb128_i16(&[0x7E]);
        assert_eq!(value, Ok((&[][..], -2)));
        let value: ResType3 = leb128_i16(&[0xFE, 0x7F]);
        assert_eq!(value, Ok((&[][..], -2)));
        let value: ResType3 = leb128_i16(&[0xFE, 0xFF, 0x7F]);
        assert_eq!(value, Ok((&[][..], -2)));
        let value: ResType3 = leb128_i16(&[0xFE, 0xFF, 0x7F, 0x00]);
        assert_eq!(value, Ok((&[0x00][..], -2)));
        let value: ResType3 = leb128_i16(&[0xFE, 0xFF, 0xFF, 0x7F, 0x00]);
        assert!(value.is_err(), "Should fail on too large input");
    }
}
