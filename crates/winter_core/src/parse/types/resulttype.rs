use nom::{error::context, multi::length_count};

use crate::parse::{values::vector_count_parser, Res};
use std::vec::Vec;

use super::{valtype_parser, ValType};

pub type ResultType = Vec<ValType>;

pub fn resulttype_parser(input: &[u8]) -> Res<&[u8], ResultType> {
    context(
        "resulttype",
        length_count(vector_count_parser, valtype_parser),
    )(input)
}
