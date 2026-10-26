use core::{error, fmt, result};

use crate::sys;

pub use sys::ErrorType;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error
{
	OutOfMemory,
	Os(ErrorType)
}

impl error::Error for Error
{
}

impl fmt::Display for Error
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			Self::OutOfMemory => f.write_str("out of memory"),
			Self::Os(code) => write!(f, "os error {code}")
		}
	}
}
