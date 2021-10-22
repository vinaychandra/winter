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
    modules::{typeidx_parser, TypeIdx},
    types::{globaltype_parser, memtype_parser, tabletype_parser, GlobalType, MemType, TableType},
    values::{name_parser, vector_count_parser},
    Res,
};

pub enum ImportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

pub struct Import<'a> {
    pub mod_name: &'a str,
    pub name: &'a str,
    pub desc: ImportDesc,
}

pub struct ImportSection<'a> {
    pub imports: Vec<Import<'a>>,
}

fn import_desc_parser(input: &[u8]) -> Res<&[u8], ImportDesc> {
    context(
        "import_desc",
        alt((
            map(preceded(tag([0x0]), typeidx_parser), ImportDesc::Func),
            map(preceded(tag([0x1]), tabletype_parser), ImportDesc::Table),
            map(preceded(tag([0x2]), memtype_parser), ImportDesc::Mem),
            map(preceded(tag([0x3]), globaltype_parser), ImportDesc::Global),
        )),
    )(input)
}

fn import_parser(input: &[u8]) -> Res<&[u8], Import> {
    context(
        "import",
        map(
            tuple((name_parser, name_parser, import_desc_parser)),
            |(mod_name, name, desc)| Import {
                mod_name,
                name,
                desc,
            },
        ),
    )(input)
}

pub fn import_section_parser(input: &[u8]) -> Res<&[u8], ImportSection> {
    context(
        "import_section",
        map(
            length_value(
                section_length_parser(2),
                length_count(vector_count_parser, import_parser),
            ),
            |imports| ImportSection { imports },
        ),
    )(input)
}
