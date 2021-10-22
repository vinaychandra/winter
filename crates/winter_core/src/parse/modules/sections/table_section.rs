use std::vec::Vec;

use nom::{
    combinator::map,
    error::context,
    multi::{length_count, length_value},
};

use super::section_length_parser;
use crate::parse::{
    types::{tabletype_parser, TableType},
    values::vector_count_parser,
    Res,
};

#[derive(Debug)]
pub struct Table {
    pub table_type: TableType,
}

#[derive(Debug)]
pub struct TableSection {
    pub tables: Vec<Table>,
}

pub fn table_section_parser(input: &[u8]) -> Res<TableSection> {
    context(
        "table_section",
        map(
            length_value(
                section_length_parser(4),
                length_count(vector_count_parser, tabletype_parser),
            ),
            |f| TableSection {
                tables: f
                    .into_iter()
                    .map(|table_type| Table { table_type })
                    .collect(),
            },
        ),
    )(input)
}
