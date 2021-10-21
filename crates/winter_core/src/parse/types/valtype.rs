use nom::{branch::alt, combinator::map, error::context};

use super::{numtype_parser, reftype_parser, NumType, RefType};
use crate::parse::Res;

/// Value types are either a [`NumType`] or [`RefType`].
/// See [`valtype`] for more information.
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ValType {
    NumType(NumType),
    RefType(RefType),
}

impl From<NumType> for ValType {
    fn from(num_type: NumType) -> Self {
        ValType::NumType(num_type)
    }
}

impl From<RefType> for ValType {
    fn from(ref_type: RefType) -> Self {
        ValType::RefType(ref_type)
    }
}

/// Value types are encoded with their respective encoding as a [`NumType`] or
/// [`RefType`].
///
/// [Reference](https://webassembly.github.io/spec/core/binary/types.html#value-types)
pub fn valtype_parser(input: &[u8]) -> Res<&[u8], ValType> {
    context(
        "valtype",
        alt((
            map(numtype_parser, ValType::from),
            map(reftype_parser, ValType::from),
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valtype() {
        type ResType<'a> = Res<&'a [u8], ValType>;

        let value: ResType = valtype_parser(&[0x7C]);
        assert_eq!(value, Ok((&[][..], ValType::NumType(NumType::F64))));
    }
}
