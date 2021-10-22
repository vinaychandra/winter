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
    modules::{funcidx_parser, tableidx_parser, FuncIdx, TableIdx},
    types::{reftype_parser, RefType},
    values::vector_count_parser,
    Res,
};

#[derive(Debug)]
pub enum ElemKind {
    FuncRef,
    RefType(RefType),
}

#[derive(Debug)]
pub enum ElemInit {
    Expr(Vec<Expr>),
    FuncIdx(Vec<FuncIdx>),
}

#[derive(Debug)]
pub enum ElemMode {
    Declarative,
    Passive,
    Active { table: TableIdx, offset: Expr },
}

#[derive(Debug)]
pub struct Elem {
    pub elem_type: ElemKind,
    pub init: ElemInit,
    pub mode: ElemMode,
}

#[derive(Debug)]
pub struct ElementSection {
    pub elems: Vec<Elem>,
}

fn elemkind_parser(input: &[u8]) -> Res<&[u8], ElemKind> {
    context(
        "elemkind",
        alt((
            map(tag([0x00]), |_| ElemKind::FuncRef),
            map(tag([0x00]), |_| ElemKind::FuncRef),
        )),
    )(input)
}

fn elem_parser(input: &[u8]) -> Res<&[u8], Elem> {
    context(
        "elem",
        alt((
            map(
                preceded(
                    tag([0x00]),
                    tuple((
                        expr_parser,
                        length_count(vector_count_parser, funcidx_parser),
                    )),
                ),
                |(expr, init)| Elem {
                    elem_type: ElemKind::FuncRef,
                    init: ElemInit::FuncIdx(init),
                    mode: ElemMode::Active {
                        offset: expr,
                        table: TableIdx(0),
                    },
                },
            ),
            map(
                preceded(
                    tag([0x01]),
                    tuple((
                        elemkind_parser,
                        length_count(vector_count_parser, funcidx_parser),
                    )),
                ),
                |(elem_type, init)| Elem {
                    elem_type,
                    init: ElemInit::FuncIdx(init),
                    mode: ElemMode::Passive,
                },
            ),
            map(
                preceded(
                    tag([0x02]),
                    tuple((
                        tableidx_parser,
                        expr_parser,
                        elemkind_parser,
                        length_count(vector_count_parser, funcidx_parser),
                    )),
                ),
                |(x, e, et, y)| Elem {
                    elem_type: et,
                    init: ElemInit::FuncIdx(y),
                    mode: ElemMode::Active {
                        offset: e,
                        table: x,
                    },
                },
            ),
            map(
                preceded(
                    tag([0x03]),
                    tuple((
                        elemkind_parser,
                        length_count(vector_count_parser, funcidx_parser),
                    )),
                ),
                |(et, y)| Elem {
                    elem_type: et,
                    init: ElemInit::FuncIdx(y),
                    mode: ElemMode::Declarative,
                },
            ),
            map(
                preceded(
                    tag([0x04]),
                    tuple((expr_parser, length_count(vector_count_parser, expr_parser))),
                ),
                |(e, el)| Elem {
                    elem_type: ElemKind::FuncRef,
                    init: ElemInit::Expr(el),
                    mode: ElemMode::Active {
                        offset: e,
                        table: TableIdx(0),
                    },
                },
            ),
            map(
                preceded(
                    tag([0x05]),
                    tuple((
                        reftype_parser,
                        length_count(vector_count_parser, expr_parser),
                    )),
                ),
                |(et, el)| Elem {
                    elem_type: ElemKind::RefType(et),
                    init: ElemInit::Expr(el),
                    mode: ElemMode::Passive,
                },
            ),
            map(
                preceded(
                    tag([0x06]),
                    tuple((
                        tableidx_parser,
                        expr_parser,
                        reftype_parser,
                        length_count(vector_count_parser, expr_parser),
                    )),
                ),
                |(x, e, et, el)| Elem {
                    elem_type: ElemKind::RefType(et),
                    init: ElemInit::Expr(el),
                    mode: ElemMode::Active {
                        offset: e,
                        table: x,
                    },
                },
            ),
            map(
                preceded(
                    tag([0x07]),
                    tuple((
                        reftype_parser,
                        length_count(vector_count_parser, expr_parser),
                    )),
                ),
                |(et, el)| Elem {
                    elem_type: ElemKind::RefType(et),
                    init: ElemInit::Expr(el),
                    mode: ElemMode::Declarative,
                },
            ),
        )),
    )(input)
}

pub fn element_section_parser(input: &[u8]) -> Res<&[u8], ElementSection> {
    context(
        "element_section",
        map(
            length_value(
                section_length_parser(9),
                length_count(vector_count_parser, elem_parser),
            ),
            |elems| ElementSection { elems },
        ),
    )(input)
}
