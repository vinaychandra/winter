use std::vec::Vec;

use nom::{
    combinator::map,
    error::context,
    multi::{length_count, length_value},
};

use super::section_length_parser;
use crate::parse::{
    modules::{typeidx_parser, TypeIdx},
    values::vector_count_parser,
    Res,
};

#[derive(Debug)]
pub struct FuncSection {
    pub functions: Vec<TypeIdx>,
}

pub fn func_section_parser(input: &[u8]) -> Res<FuncSection> {
    context(
        "func_section",
        map(
            length_value(
                section_length_parser(3),
                length_count(vector_count_parser, typeidx_parser),
            ),
            |functions| FuncSection { functions },
        ),
    )(input)
}
