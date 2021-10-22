use std::vec::Vec;

use nom::{
    combinator::map,
    error::context,
    multi::{length_count, length_value},
};

use super::section_length_parser;
use crate::parse::{
    types::{memtype_parser, MemType},
    values::vector_count_parser,
    Res,
};

#[derive(Debug)]
pub struct Mem {
    pub mem_type: MemType,
}

#[derive(Debug)]
pub struct MemSection {
    pub tables: Vec<Mem>,
}

pub fn memory_section_parser(input: &[u8]) -> Res<MemSection> {
    context(
        "memory_section",
        map(
            length_value(
                section_length_parser(5),
                length_count(vector_count_parser, memtype_parser),
            ),
            |f| MemSection {
                tables: f.into_iter().map(|mem_type| Mem { mem_type }).collect(),
            },
        ),
    )(input)
}
