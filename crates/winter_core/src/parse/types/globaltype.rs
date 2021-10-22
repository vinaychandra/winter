use nom::{bytes::complete::take, combinator::map, error::context, sequence::pair};

use super::{valtype_parser, ValType};
use crate::parse::Res;

/// GlobalType.
#[derive(Debug)]
pub struct GlobalType {
    pub value_type: ValType,

    /// Is mutable or a const.
    pub mutable: bool,
}

/// Global types are encoded by their value type and a flag for their
/// mutability.
///
/// [Reference](https://webassembly.github.io/spec/core/binary/types.html#global-types)
pub fn globaltype_parser(input: &[u8]) -> Res<GlobalType> {
    context(
        "globaltype",
        map(pair(valtype_parser, take(1usize)), |(value_type, b)| {
            GlobalType {
                value_type,
                mutable: b[0] == 0x01,
            }
        }),
    )(input)
}
