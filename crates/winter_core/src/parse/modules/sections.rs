use nom::{bytes::complete::tag, sequence::preceded};

use crate::parse::{values::vector_count_parser, Res};

pub mod code_section;
pub mod custom_section;
pub mod data_count_section;
pub mod data_section;
pub mod element_section;
pub mod export_section;
pub mod function_section;
pub mod global_section;
pub mod import_section;
pub mod memory_section;
pub mod module;
pub mod start_section;
pub mod table_section;
pub mod type_section;

/// Parser a section. Validates the section number and returns
/// the number of bytes for the section.
fn section_length_parser(section_id: u8) -> impl FnMut(&[u8]) -> Res<u32> {
    move |inp: &[u8]| preceded(tag([section_id]), vector_count_parser)(inp)
}
