use nom::{bytes::complete::take, combinator::map_res, error::context};

use crate::parse::Res;

/// Reference types are encoded by a single byte.
/// [Reference](https://webassembly.github.io/spec/core/binary/types.html#reference-types)
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

impl TryFrom<u8> for RefType {
    type Error = &'static str;

    fn try_from(input: u8) -> Result<Self, Self::Error> {
        match input {
            0x70 => Ok(RefType::FuncRef),
            0x6F => Ok(RefType::ExternRef),
            _ => Err("Invalid ref type"),
        }
    }
}

/// Reference types are encoded by a single byte.
/// [Reference](https://webassembly.github.io/spec/core/binary/types.html#reference-types)
/// Returns a struct `RefType` which contains the type of the reference.
pub fn reftype_parser(input: &[u8]) -> Res<RefType> {
    context(
        "reftype",
        map_res(take(1usize), |f: &[u8]| (f[0]).try_into()),
    )(input)
}
