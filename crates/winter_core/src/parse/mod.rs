use nom::{error::VerboseError, IResult};

pub mod instructions;
pub mod modules;
pub mod types;
pub mod values;

pub type Res<T, U> = IResult<T, U, VerboseError<T>>;
