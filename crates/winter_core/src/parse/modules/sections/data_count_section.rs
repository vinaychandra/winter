use nom::{combinator::map, error::context, multi::length_value};

use super::section_length_parser;
use crate::parse::{values::leb128_u32, Res};

pub struct DataCountSection {
    pub count: u32,
}

pub fn data_count_section_parser(input: &[u8]) -> Res<&[u8], DataCountSection> {
    context(
        "data_count",
        map(
            length_value(section_length_parser(12), leb128_u32),
            |count| DataCountSection { count },
        ),
    )(input)
}
