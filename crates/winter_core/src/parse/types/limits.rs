use nom::{branch::alt, bytes::complete::tag, combinator::map, error::context, sequence::tuple};

use super::super::values::leb128_u32;
use crate::parse::Res;

/// Limits with a required min value and an optional max value.
#[derive(Debug)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

/// Limits are encoded with a preceding flag indicating whether a maximum is
/// present.
///
/// [Reference](https://webassembly.github.io/spec/core/binary/types.html#limits)
pub fn limits_parser(input: &[u8]) -> Res<&[u8], Limits> {
    context(
        "limits",
        alt((
            map(tuple((tag([0x0]), leb128_u32)), |(_, min)| Limits {
                min,
                max: None,
            }),
            map(
                tuple((tag([0x0]), leb128_u32, leb128_u32)),
                |(_, min, max)| Limits {
                    min,
                    max: Some(max),
                },
            ),
        )),
    )(input)
}
