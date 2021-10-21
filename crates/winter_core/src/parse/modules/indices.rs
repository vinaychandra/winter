//! [Reference](https://webassembly.github.io/spec/core/binary/modules.html#indices)
//! Definitions are referenced with zero-based indices. Each class of definition has its own index space.

use crate::parse::{values::leb128_u32, Res};
use nom::combinator::map;

macro_rules! impl_idx {
    ($typename: ident, $parse_fn: ident) => {
        pub struct $typename(pub u32);

        pub fn $parse_fn(input: &[u8]) -> Res<&[u8], $typename> {
            map(leb128_u32, |f| $typename(f))(input)
        }

        impl From<u32> for $typename {
            fn from(f: u32) -> Self {
                $typename(f)
            }
        }
    };
}

impl_idx!(TypeIdx, typeidx_parser);
impl_idx!(FuncIdx, funcidx_parser);
impl_idx!(TableIdx, tableidx_parser);
impl_idx!(MemIdx, memidx_parser);
impl_idx!(GlobalIdx, globalidx_parser);
impl_idx!(ElemIdx, elemidx_parser);
impl_idx!(DataIdx, dataidx_parser);
impl_idx!(LocalIdx, localidx_parser);
impl_idx!(LabelIdx, labelidx_parser);
