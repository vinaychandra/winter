use std::vec::Vec;

use nom::{
    bytes::complete::tag,
    combinator::opt,
    error::{context, VerboseError},
    multi::many0,
    sequence::tuple,
};

use super::{
    code_section::{code_section_parser, CodeSection},
    custom_section::{custom_section_parser, CustomSection},
    data_count_section::{data_count_section_parser, DataCountSection},
    data_section::{data_section_parser, DataSection},
    element_section::{element_section_parser, ElementSection},
    export_section::{export_section_parser, ExportSection},
    function_section::{func_section_parser, FuncSection},
    global_section::{global_section_parser, GlobalSection},
    import_section::{import_section_parser, ImportSection},
    memory_section::{memory_section_parser, MemSection},
    start_section::{start_section_parser, StartSection},
    table_section::{table_section_parser, TableSection},
    type_section::{type_section_parser, TypeSection},
};

#[derive(Default)]
pub struct Module<'a> {
    pub custom_sections: Vec<CustomSection<'a>>,
    pub func_type: Option<TypeSection>,
    pub import: Option<ImportSection<'a>>,
    pub type_idx: Option<FuncSection>,
    pub table: Option<TableSection>,
    pub mem: Option<MemSection>,
    pub global: Option<GlobalSection>,
    pub export: Option<ExportSection<'a>>,
    pub start: Option<StartSection>,
    pub elem: Option<ElementSection>,
    pub m: Option<DataCountSection>,
    pub code: Option<CodeSection>,
    pub data: Option<DataSection<'a>>,
}

pub fn module_parser(input: &[u8]) -> Result<Module, nom::Err<VerboseError<&[u8]>>> {
    let result = tuple((
        context("magic", tag([0x00, 0x61, 0x73, 0x6D])),
        context("version", tag([0x01, 0x00, 0x00, 0x00])),
        many0(custom_section_parser),
        opt(type_section_parser),
        many0(custom_section_parser),
        opt(import_section_parser),
        many0(custom_section_parser),
        opt(func_section_parser),
        many0(custom_section_parser),
        opt(table_section_parser),
        many0(custom_section_parser),
        opt(memory_section_parser),
        many0(custom_section_parser),
        opt(global_section_parser),
        many0(custom_section_parser),
        opt(export_section_parser),
        many0(custom_section_parser),
        opt(start_section_parser),
        tuple((
            many0(custom_section_parser),
            opt(element_section_parser),
            many0(custom_section_parser),
            opt(data_count_section_parser),
            many0(custom_section_parser),
            opt(code_section_parser),
            many0(custom_section_parser),
            opt(data_section_parser),
            many0(custom_section_parser),
        )),
    ))(input)?
    .1;

    let mut m = Module {
        func_type: result.3,
        import: result.5,
        type_idx: result.7,
        table: result.9,
        mem: result.11,
        global: result.13,
        export: result.15,
        start: result.17,
        elem: result.18 .1,
        m: result.18 .3,
        code: result.18 .5,
        data: result.18 .7,
        ..Default::default()
    };

    m.custom_sections.extend(result.2.into_iter());
    m.custom_sections.extend(result.4.into_iter());
    m.custom_sections.extend(result.6.into_iter());
    m.custom_sections.extend(result.8.into_iter());
    m.custom_sections.extend(result.10.into_iter());
    m.custom_sections.extend(result.12.into_iter());
    m.custom_sections.extend(result.14.into_iter());
    m.custom_sections.extend(result.16.into_iter());
    m.custom_sections.extend(result.18 .0.into_iter());
    m.custom_sections.extend(result.18 .2.into_iter());
    m.custom_sections.extend(result.18 .4.into_iter());
    m.custom_sections.extend(result.18 .6.into_iter());
    m.custom_sections.extend(result.18 .8.into_iter());

    Ok(m)
}
