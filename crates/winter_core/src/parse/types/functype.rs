use nom::{bytes::complete::tag, error::context, sequence::tuple};

use super::{resulttype_parser, ResultType};
use crate::parse::Res;

/// Function type info.
#[derive(Debug)]
pub struct FuncType {
    pub params: ResultType,
    pub ret: ResultType,
}

/// Function types are encoded by the byte 𝟶𝚡𝟼𝟶 followed by the respective
/// vectors of parameter and result types.
///
/// [Reference](https://webassembly.github.io/spec/core/binary/types.html#function-types)
pub fn functype_parser(input: &[u8]) -> Res<FuncType> {
    let r = context(
        "functype",
        tuple((tag([0x60]), resulttype_parser, resulttype_parser)),
    )(input)?;
    Ok((
        r.0,
        FuncType {
            params: r.1 .1,
            ret: r.1 .2,
        },
    ))
}
