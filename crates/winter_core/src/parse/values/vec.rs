use nom::bytes::complete::take;

use super::leb128_u32;
use crate::parse::Res;

/// Vectors are encoded with their `u32` length followed by the encoding of their element sequence.
/// [Reference](https://webassembly.github.io/spec/core/binary/conventions.html#vectors).
/// `u32` is decoded by [`leb128_u32`].
pub fn vector(input: &[u8]) -> Res<&[u8], &[u8]> {
    leb128_u32(input).and_then(|(remaining, length)| take(length)(remaining))
}

#[cfg(test)]
mod tests {
    use crate::parse::Res;

    use super::*;

    #[test]
    fn test_vector_data() {
        type ResType<'a> = Res<&'a [u8], &'a [u8]>;

        let value: ResType = vector(&[0x1, 0x2, 0x3, 0x4]);
        assert_eq!(value, Ok((&[0x3, 0x4][..], &[0x2][..])));

        let value: ResType = vector(&[0x2, 0x2, 0x3, 0x4]);
        assert_eq!(value, Ok((&[0x4][..], &[0x2, 0x3][..])));

        let value: ResType = vector(&[0x1]);
        assert!(value.is_err());
    }
}
