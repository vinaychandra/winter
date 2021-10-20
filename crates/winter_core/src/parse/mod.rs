use nom::{error::VerboseError, IResult};

pub mod values;

pub type Res<T, U> = IResult<T, U, VerboseError<T>>;
