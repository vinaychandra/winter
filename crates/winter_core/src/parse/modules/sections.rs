use nom::{bytes::complete::tag, sequence::preceded};

use crate::parse::{values::vector_count_parser, Res};

pub mod custom_section;
pub mod import_section;
pub mod type_section;

pub enum SectionType {
    CustomSection,
    TypeSection,
    ImportSection,
    FunctionSection,
    TableSection,
    MemorySection,
    GlobalSection,
    ExportSection,
    StartSection,
    ElementSection,
    CodeSection,
    DataSection,
    DataCountSection,
}

/// Parser a section. Validates the section number and returns
/// the number of bytes for the section.
fn section_length_parser(section_id: u8) -> impl FnMut(&[u8]) -> Res<&[u8], u32> {
    move |inp: &[u8]| preceded(tag([section_id]), vector_count_parser)(inp)
}
