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
    modules::{
        funcidx_parser, globalidx_parser, memidx_parser, tableidx_parser, FuncIdx, GlobalIdx,
        MemIdx, TableIdx,
    },
    values::{name_parser, vector_count_parser},
    Res,
};

pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

pub struct Export<'a> {
    pub name: &'a str,
    pub desc: ExportDesc,
}

pub struct ExportSec<'a> {
    pub exports: Vec<Export<'a>>,
}

fn export_desc_parser(input: &[u8]) -> Res<&[u8], ExportDesc> {
    context(
        "export_desc",
        alt((
            map(preceded(tag([0x0]), funcidx_parser), ExportDesc::Func),
            map(preceded(tag([0x1]), tableidx_parser), ExportDesc::Table),
            map(preceded(tag([0x2]), memidx_parser), ExportDesc::Mem),
            map(preceded(tag([0x3]), globalidx_parser), ExportDesc::Global),
        )),
    )(input)
}

fn export_parser(input: &[u8]) -> Res<&[u8], Export> {
    context(
        "export",
        map(tuple((name_parser, export_desc_parser)), |(name, desc)| {
            Export { name, desc }
        }),
    )(input)
}

pub fn export_section_parser(input: &[u8]) -> Res<&[u8], ExportSec> {
    context(
        "export_section",
        map(
            length_value(
                section_length_parser(7),
                length_count(vector_count_parser, export_parser),
            ),
            |exports| ExportSec { exports },
        ),
    )(input)
}
