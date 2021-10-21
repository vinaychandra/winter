mod functype;
mod globaltype;
mod limits;
mod numtype;
mod reftype;
mod resulttype;
mod tabletype;
mod valtype;

pub use functype::*;
pub use globaltype::*;
pub use limits::*;
pub use numtype::*;
pub use reftype::*;
pub use resulttype::*;
pub use tabletype::*;
pub use valtype::*;

pub type MemType = Limits;
pub use self::limits_parser as memtype_parser;
