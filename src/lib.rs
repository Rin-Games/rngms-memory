#![no_std]

mod sys;

mod region;
mod error;
mod protection;

pub use region::Region;
pub use error::{Error, ErrorKind, ErrorType, Result};
pub use protection::{Protection, ProtectionType};
pub use sys::{page_size, allocation_granularity};
