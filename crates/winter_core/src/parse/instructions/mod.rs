mod control;
mod expr;

use std::boxed::Box;

pub use control::*;
pub use expr::*;
use nom::{branch::alt, combinator::map, error::context};

use super::Res;

#[derive(Debug)]
pub enum Instr {
    Control(Box<ControlInstruction>),
}

pub fn instr_parser(input: &[u8]) -> Res<Instr> {
    context(
        "instr",
        alt((
            map(control_instr_parser, |f| Instr::Control(Box::new(f))),
            map(control_instr_parser, |f| Instr::Control(Box::new(f))),
        )),
    )(input)
}
