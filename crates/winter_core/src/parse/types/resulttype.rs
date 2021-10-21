use nom::{combinator::flat_map, error::context, multi::count};

use crate::parse::{values::vector_count_parser, Res};
use std::vec::Vec;

use super::{valtype_parser, ValType};

pub type ResultType = Vec<ValType>;

pub fn resulttype_parser(input: &[u8]) -> Res<&[u8], ResultType> {
    context(
        "resulttype",
        flat_map(vector_count_parser, |c| count(valtype_parser, c as usize)),
    )(input)
}
