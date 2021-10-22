use std::vec::Vec;

use nom::{
    combinator::map,
    error::context,
    multi::{length_count, length_value},
    sequence::tuple,
};

use super::section_length_parser;
use crate::parse::{
    instructions::{expr_parser, Expr},
    types::{globaltype_parser, GlobalType},
    values::vector_count_parser,
    Res,
};

#[derive(Debug)]
pub struct Global {
    pub global_type: GlobalType,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct GlobalSection {
    pub globals: Vec<Global>,
}

fn global_parser(input: &[u8]) -> Res<&[u8], Global> {
    context(
        "global",
        map(
            tuple((globaltype_parser, expr_parser)),
            |(global_type, expr)| Global { global_type, expr },
        ),
    )(input)
}

pub fn global_section_parser(input: &[u8]) -> Res<&[u8], GlobalSection> {
    context(
        "global_section",
        map(
            length_value(
                section_length_parser(6),
                length_count(vector_count_parser, global_parser),
            ),
            |globals| GlobalSection { globals },
        ),
    )(input)
}
