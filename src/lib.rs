#![no_std]

mod sys;

mod region;
mod error;
mod protection;

pub use region::Region;
pub use error::{Error, Result, ErrorType};
pub use protection::{Protection, ProtectionType};
