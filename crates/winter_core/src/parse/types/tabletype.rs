use nom::{combinator::map, error::context, sequence::pair};

use crate::parse::Res;

use super::{limits_parser, reftype_parser, Limits, RefType};

/// Table type info.
pub struct TableType {
    pub ref_type: RefType,
    pub limits: Limits,
}

/// Table types are encoded with their limits and the encoding of their element reference type.
/// [Reference](https://webassembly.github.io/spec/core/binary/types.html#table-types)
pub fn tabletype_parser(input: &[u8]) -> Res<&[u8], TableType> {
    map(
        context("tabletype", pair(reftype_parser, limits_parser)),
        |(reftype, limits)| TableType {
            ref_type: reftype,
            limits,
        },
    )(input)
}
