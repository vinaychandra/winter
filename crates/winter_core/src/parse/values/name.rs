use nom::{combinator::map_res, error::context};

use super::vector_parser;
use crate::parse::Res;

/// Names are encoded as a vector of bytes containing the Unicode
/// UTF-8 encoding of the name’s character sequence.
///
/// [Reference](https://webassembly.github.io/spec/core/binary/values.html#names)
pub fn name_parser(input: &[u8]) -> Res<&[u8], &str> {
    context("name", map_res(vector_parser, core::str::from_utf8))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Res;

    #[test]
    fn test_name() {
        type ResType<'a> = Res<&'a [u8], &'a str>;

        let value: ResType = name_parser(&[0x2, 0x41, 0x42, 0x43]);
        assert_eq!(value, Ok((&[0x43][..], "AB")));
    }
}
