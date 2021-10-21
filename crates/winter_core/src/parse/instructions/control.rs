use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    error::context,
    multi::{length_count, many0},
    sequence::{delimited, preceded, tuple},
};

use crate::parse::{
    modules::{
        funcidx_parser, labelidx_parser, tableidx_parser, typeidx_parser, FuncIdx, LabelIdx,
        TableIdx, TypeIdx,
    },
    types::{valtype_parser, ValType},
    values::{leb128_i33, vector_count_parser},
    Res,
};

use std::vec::Vec;

use super::{instr_parser, Instr};

pub enum BlockType {
    Empty,
    ValType(ValType),
    TypeIndex(TypeIdx),
}

pub fn blocktype_parser(input: &[u8]) -> Res<&[u8], BlockType> {
    context(
        "blocktype",
        alt((
            map(tag([0x40]), |_| BlockType::Empty),
            map(valtype_parser, BlockType::ValType),
            map(leb128_i33, |t| BlockType::TypeIndex((t as u32).into())), // TODO: check if this is correct
        )),
    )(input)
}

/// Instructions in this group affect the flow of control.
pub enum ControlInstruction {
    /// The unreachable instruction causes an unconditional trap.
    Unreachable,

    /// The nop instruction does nothing.
    Nop,

    Block(BlockType, Vec<Instr>),
    Loop(BlockType, Vec<Instr>),

    /// If Condition with an optional else branch.
    If(BlockType, Vec<Instr>, Vec<Instr>),

    /// Unconditional branch.
    Br(LabelIdx),
    /// Branch if Condition.
    BrIf(LabelIdx),
    /// br_table performs an indirect branch through an operand indexing
    /// into the label vector that is an immediate to the instruction,
    /// or to a default target if the operand is out of bounds.
    BrTable(Vec<LabelIdx>, LabelIdx),
    /// The return instruction is a shortcut for an unconditional branch
    /// to the outermost block, which implicitly is the body of the
    /// current function.
    Return,
    /// The call instruction invokes another function, consuming the
    /// necessary arguments from the stack and returning the result values
    /// of the call.
    Call(FuncIdx),
    /// The call_indirect instruction calls a function indirectly through an
    /// operand indexing into a table that is denoted by a table index and
    /// must have type funcref. Since it may contain functions of
    /// heterogeneous type, the callee is dynamically checked against the
    /// function type indexed by the instruction’s second immediate, and
    /// the call is aborted with a trap if it does not match.
    CallIndirect(TypeIdx, TableIdx),
}

pub fn control_instr_parser(input: &[u8]) -> Res<&[u8], ControlInstruction> {
    context(
        "conditional_instr",
        alt((
            context(
                "unreachable",
                map(tag([0x00]), |_| ControlInstruction::Unreachable),
            ),
            context("nop", map(tag([0x01]), |_| ControlInstruction::Nop)),
            context(
                "block",
                map(
                    delimited(
                        tag([0x02]),
                        tuple((blocktype_parser, many0(instr_parser))),
                        tag([0x0B]),
                    ),
                    |(b, v)| ControlInstruction::Block(b, v),
                ),
            ),
            context(
                "loop",
                map(
                    delimited(
                        tag([0x03]),
                        tuple((blocktype_parser, many0(instr_parser))),
                        tag([0x0B]),
                    ),
                    |(b, v)| ControlInstruction::Loop(b, v),
                ),
            ),
            context(
                "if_without_else",
                map(
                    delimited(
                        tag([0x04]),
                        tuple((blocktype_parser, many0(instr_parser))),
                        tag([0x0B]),
                    ),
                    |(b, v)| ControlInstruction::If(b, v, Vec::with_capacity(0)),
                ),
            ),
            context(
                "if_with_else",
                map(
                    delimited(
                        tag([0x04]),
                        tuple((
                            blocktype_parser,
                            many0(instr_parser),
                            tag([0x05]),
                            many0(instr_parser),
                        )),
                        tag([0x0B]),
                    ),
                    |(b, v, _, e)| ControlInstruction::If(b, v, e),
                ),
            ),
            context(
                "br",
                map(
                    preceded(tag([0x0C]), labelidx_parser),
                    ControlInstruction::Br,
                ),
            ),
            context(
                "br_if",
                map(
                    preceded(tag([0x0D]), labelidx_parser),
                    ControlInstruction::BrIf,
                ),
            ),
            context(
                "br_table",
                map(
                    preceded(
                        tag([0x0E]),
                        tuple((
                            length_count(vector_count_parser, labelidx_parser),
                            labelidx_parser,
                        )),
                    ),
                    |(v, d)| ControlInstruction::BrTable(v, d),
                ),
            ),
            context("return", map(tag([0x0F]), |_| ControlInstruction::Return)),
            context(
                "call",
                map(
                    preceded(tag([0x10]), funcidx_parser),
                    ControlInstruction::Call,
                ),
            ),
            context(
                "call_indirect",
                map(
                    preceded(tag([0x11]), tuple((typeidx_parser, tableidx_parser))),
                    |(t, tbl)| ControlInstruction::CallIndirect(t, tbl),
                ),
            ),
        )),
    )(input)
}
