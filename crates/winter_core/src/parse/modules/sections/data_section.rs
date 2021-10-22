use std::vec::Vec;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    error::context,
    multi::{length_count, length_value},
    sequence::{preceded, tuple},
};

use super::section_length_parser;
use crate::parse::{
    instructions::{expr_parser, Expr},
    modules::{memidx_parser, MemIdx},
    values::{vector_count_parser, vector_parser},
    Res,
};

#[derive(Debug)]
pub enum DataMode {
    Passive,
    Active { memory: MemIdx, offset: Expr },
}

#[derive(Debug)]
pub struct Data<'a> {
    pub init: &'a [u8],
    pub mode: DataMode,
}

#[derive(Debug)]
pub struct DataSection<'a> {
    pub data: Vec<Data<'a>>,
}

fn data_parser(input: &[u8]) -> Res<&[u8], Data> {
    context(
        "data",
        alt((
            map(
                preceded(tag([0x0]), tuple((expr_parser, vector_parser))),
                |(offset, init)| Data {
                    init,
                    mode: DataMode::Active {
                        offset,
                        memory: MemIdx(0),
                    },
                },
            ),
            map(preceded(tag([0x1]), vector_parser), |vec| Data {
                init: vec,
                mode: DataMode::Passive,
            }),
            map(
                preceded(
                    tag([0x2]),
                    tuple((memidx_parser, expr_parser, vector_parser)),
                ),
                |(memory, offset, init)| Data {
                    init,
                    mode: DataMode::Active { offset, memory },
                },
            ),
        )),
    )(input)
}

pub fn data_section_parser(input: &[u8]) -> Res<&[u8], DataSection> {
    context(
        "data_section",
        map(
            length_value(
                section_length_parser(11),
                length_count(vector_count_parser, data_parser),
            ),
            |data| DataSection { data },
        ),
    )(input)
}
