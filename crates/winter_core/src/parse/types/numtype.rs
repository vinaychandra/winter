use nom::{
    error::{make_error, ContextError, ErrorKind, VerboseError},
    InputTake,
};

use crate::parse::Res;

/// Number types are encoded by a single byte.
/// [Reference](https://webassembly.github.io/spec/core/binary/types.html#number-types)
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
pub fn numtype(input: &[u8]) -> Res<&[u8], NumType> {
    let value = input.take(1);
    let result: Result<NumType, &'static str> = value[0].try_into();

    result
        .map_err(|e| {
            nom::Err::Error(VerboseError::add_context(
                input,
                e,
                make_error(input, ErrorKind::Char),
            ))
        })
        .map(|v| (input, v))
}
