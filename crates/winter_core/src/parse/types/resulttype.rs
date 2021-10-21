use std::vec::Vec;

use nom::{error::context, multi::length_count};

use super::{valtype_parser, ValType};
use crate::parse::{values::vector_count_parser, Res};

pub type ResultType = Vec<ValType>;

pub fn resulttype_parser(input: &[u8]) -> Res<&[u8], ResultType> {
    context(
        "resulttype",
        length_count(vector_count_parser, valtype_parser),
    )(input)
}
