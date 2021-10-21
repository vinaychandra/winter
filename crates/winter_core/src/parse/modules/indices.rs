//! [Reference](https://webassembly.github.io/spec/core/binary/modules.html#indices)
//! Definitions are referenced with zero-based indices. Each class of definition has its own index space.

use crate::parse::{values::leb128_u32, Res};

macro_rules! impl_idx {
    ($typename: ident, $parse_fn: ident) => {
        pub type $typename = u32;

        pub fn $parse_fn(input: &[u8]) -> Res<&[u8], $typename> {
            leb128_u32(input)
        }
    };
}

impl_idx!(TypeIdx, typeidx_parse);
impl_idx!(FuncIdx, funcidx_parse);
impl_idx!(TableIdx, tableidx_parse);
impl_idx!(MemIdx, memidx_parse);
impl_idx!(GlobalIdx, globalidx_parse);
impl_idx!(ElemIdx, elemidx_parse);
impl_idx!(DataIdx, dataidx_parse);
impl_idx!(LocalIdx, localidx_parse);
impl_idx!(LabelIdx, labelidx_parse);
