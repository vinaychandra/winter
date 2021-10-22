use nom::{bytes::complete::take, combinator::map_res, error::context};

use crate::parse::Res;

/// Number types are encoded by a single byte.
/// [Reference](https://webassembly.github.io/spec/core/binary/types.html#number-types)
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

impl TryFrom<u8> for NumType {
    type Error = &'static str;

    fn try_from(input: u8) -> Result<Self, Self::Error> {
        match input {
            0x7F => Ok(NumType::I32),
            0x7E => Ok(NumType::I64),
            0x7D => Ok(NumType::F32),
            0x7C => Ok(NumType::F64),
            _ => Err("Invalid num type"),
        }
    }
}

/// Number types are encoded by a single byte.
/// [Reference](https://webassembly.github.io/spec/core/binary/types.html#number-types)
/// Returns a struct `NumType` which contains the type of the number.
pub fn numtype_parser(input: &[u8]) -> Res<NumType> {
    context(
        "numtype",
        map_res(take(1usize), |f: &[u8]| (f[0]).try_into()),
    )(input)
}
