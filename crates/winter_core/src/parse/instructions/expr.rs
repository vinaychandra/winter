use std::vec::Vec;

use nom::{
    bytes::complete::tag, combinator::map, error::context, multi::many0, sequence::terminated,
};

use super::{instr_parser, Instr};
use crate::parse::Res;

#[derive(Debug)]
pub struct Expr {
    pub instr: Vec<Instr>,
}

pub fn expr_parser(input: &[u8]) -> Res<Expr> {
    context(
        "expr",
        map(terminated(many0(instr_parser), tag([0x0B])), |instr| Expr {
            instr,
        }),
    )(input)
}
