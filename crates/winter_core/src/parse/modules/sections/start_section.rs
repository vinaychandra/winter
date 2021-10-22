use nom::{combinator::map, error::context, multi::length_value};

use super::section_length_parser;
use crate::parse::{
    modules::{funcidx_parser, FuncIdx},
    Res,
};

#[derive(Debug)]
pub struct StartSection {
    pub start: FuncIdx,
}

pub fn start_section_parser(input: &[u8]) -> Res<StartSection> {
    context(
        "start_section",
        map(
            length_value(section_length_parser(8), funcidx_parser),
            |start| StartSection { start },
        ),
    )(input)
}
