use nom::{error::VerboseError, IResult};

pub mod instructions;
pub mod modules;
pub mod types;
pub mod values;

pub type Res<'a, U> = IResult<&'a [u8], U, VerboseError<&'a [u8]>>;
