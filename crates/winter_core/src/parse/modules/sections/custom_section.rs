use nom::{
    bytes::complete::take_while, combinator::map, error::context, multi::length_value,
    sequence::tuple,
};

use super::section_length_parser;
use crate::parse::{values::name_parser, Res};

#[derive(Debug)]
pub struct CustomSection<'a> {
    pub name: &'a str,
    pub data: &'a [u8],
}

pub fn custom_section_parser(input: &[u8]) -> Res<&[u8], CustomSection> {
    context(
        "custom_section",
        map(
            length_value(
                section_length_parser(0),
                tuple((name_parser, take_while(|_| true))),
            ),
            |(name, data)| CustomSection { name, data },
        ),
    )(input)
}
