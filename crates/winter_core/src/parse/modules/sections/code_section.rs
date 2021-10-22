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
    types::{valtype_parser, ValType},
    values::{leb128_u32, vector_count_parser},
    Res,
};

pub struct Locals {
    pub count: u32,
    pub val_type: ValType,
}

pub struct Func {
    pub locals: Vec<Locals>,
    pub expr: Expr,
}

pub struct Code {
    pub size: u32,
    pub code: Func,
}

pub struct CodeSection {
    pub code: Vec<Code>,
}

fn locals_parser(input: &[u8]) -> Res<&[u8], Locals> {
    context(
        "locals",
        map(tuple((leb128_u32, valtype_parser)), |(count, val_type)| {
            Locals { count, val_type }
        }),
    )(input)
}

fn func_parser(input: &[u8]) -> Res<&[u8], Func> {
    context(
        "func",
        map(
            tuple((
                length_count(vector_count_parser, locals_parser),
                expr_parser,
            )),
            |(locals, expr)| Func { locals, expr },
        ),
    )(input)
}

fn code_parser(input: &[u8]) -> Res<&[u8], Code> {
    context(
        "code",
        map(tuple((leb128_u32, func_parser)), |(size, code)| Code {
            size,
            code,
        }),
    )(input)
}

pub fn code_section_parser(input: &[u8]) -> Res<&[u8], CodeSection> {
    context(
        "code_section",
        map(
            length_value(
                section_length_parser(10),
                length_count(vector_count_parser, code_parser),
            ),
            |code| CodeSection { code },
        ),
    )(input)
}