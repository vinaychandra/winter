use nom::{branch::alt, bytes::complete::tag, combinator::map, error::context};

use crate::parse::{
    modules::TypeIdx,
    types::{valtype_parser, ValType},
    values::leb128_i33,
    Res,
};

pub enum BlockType {
    Empty,
    ValType(ValType),
    TypeIndex(TypeIdx),
}

pub fn blocktype_parser(input: &[u8]) -> Res<&[u8], BlockType> {
    context(
        "blocktype",
        alt((
            map(tag([0x40]), |_| BlockType::Empty),
            map(valtype_parser, BlockType::ValType),
            map(leb128_i33, |t| BlockType::TypeIndex(t as u32)), // TODO: check if this is correct
        )),
    )(input)
}
