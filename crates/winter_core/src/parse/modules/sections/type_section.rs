use std::vec::Vec;

use nom::{
    combinator::map,
    error::context,
    multi::{length_count, length_value},
};

use super::section_length_parser;
use crate::parse::{
    types::{functype_parser, FuncType},
    values::vector_count_parser,
    Res,
};

pub struct TypeSection {
    pub function_types: Vec<FuncType>,
}

pub fn type_section_parser(input: &[u8]) -> Res<&[u8], TypeSection> {
    context(
        "type_section",
        map(
            length_value(
                section_length_parser(1),
                length_count(vector_count_parser, functype_parser),
            ),
            |function_types| TypeSection { function_types },
        ),
    )(input)
}
