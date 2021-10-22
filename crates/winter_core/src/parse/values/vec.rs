use std::ops::RangeFrom;

use nom::{
    bytes::complete::take, error::VerboseError, IResult, InputIter, InputLength, InputTake, Slice,
};

use super::leb128_u32;

/// Vectors are encoded with their `u32` length followed by the encoding of
/// their element sequence.
///
/// [Reference](https://webassembly.github.io/spec/core/binary/conventions.html#vectors).
/// `u32` is decoded by [`leb128_u32`].
pub fn vector_parser<I>(input: I) -> IResult<I, I, VerboseError<I>>
where
    I: Clone + Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength + InputTake,
{
    leb128_u32(input).and_then(|(remaining, length)| take(length)(remaining))
}

/// Vectors are encoded with their `u32` length followed by the encoding of
/// their element sequence.
///
/// [Reference](https://webassembly.github.io/spec/core/binary/conventions.html#vectors).
///
/// `u32` is decoded by [`leb128_u32`].
/// This method returns the length of the vector.
pub fn vector_count_parser<I>(input: I) -> IResult<I, u32, VerboseError<I>>
where
    I: Clone + Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength + InputTake,
{
    leb128_u32(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Res;

    #[test]
    fn test_vector_data() {
        type ResType<'a> = Res<'a, &'a [u8]>;

        let value: ResType = vector_parser(&[0x1, 0x2, 0x3, 0x4]);
        assert_eq!(value, Ok((&[0x3, 0x4][..], &[0x2][..])));

        let value: ResType = vector_parser(&[0x2, 0x2, 0x3, 0x4]);
        assert_eq!(value, Ok((&[0x4][..], &[0x2, 0x3][..])));

        let value: ResType = vector_parser(&[0x1]);
        assert!(value.is_err());
    }
}
