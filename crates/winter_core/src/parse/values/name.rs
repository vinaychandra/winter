use super::vector;
use crate::parse::Res;
use nom::error::{make_error, ContextError, ErrorKind, VerboseError};

/// Names are encoded as a vector of bytes containing the Unicode
/// UTF-8 encoding of the name’s character sequence.
/// [Reference](https://webassembly.github.io/spec/core/binary/values.html#names)
pub fn name_parser(input: &[u8]) -> Res<&[u8], &str> {
    vector(input).and_then(|(remaining, data)| {
        core::str::from_utf8(data)
            .map_err(|_| {
                nom::Err::Error(VerboseError::add_context(
                    input,
                    "Unicode decoding failed",
                    make_error(input, ErrorKind::Verify),
                ))
            })
            .map(|inp| (remaining, inp))
    })
}

#[cfg(test)]
mod tests {
    use crate::parse::Res;

    use super::*;

    #[test]
    fn test_name() {
        type ResType<'a> = Res<&'a [u8], &'a str>;

        let value: ResType = name_parser(&[0x2, 0x41, 0x42, 0x43]);
        assert_eq!(value, Ok((&[0x43][..], "AB")));
    }
}
