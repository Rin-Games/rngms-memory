use core::{error, fmt, result};

use crate::sys;

pub use sys::ErrorType;

pub type Result<T> = result::Result<T, Error>;

#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ErrorKind
{
	OutOfMemory,
	OutOfAddressSpace,
	LockLimit,
	PermissionDenied,
	InvalidAddress,
	InvalidArgument,
	Unsupported,
	Other
}

impl ErrorKind
{
	pub const fn as_str(self) -> &'static str
	{
		match self
		{
			Self::OutOfMemory => "out of memory",
			Self::OutOfAddressSpace => "out of address space",
			Self::LockLimit => "lock limit exceeded",
			Self::PermissionDenied => "permission denied",
			Self::InvalidAddress => "invalid address",
			Self::InvalidArgument => "invalid argument",
			Self::Unsupported => "unsupported",
			Self::Other => "other"
		}
	}
}

impl fmt::Display for ErrorKind
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		f.write_str(self.as_str())
	}
}

// operation that failed, the backend maps os codes per operation
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Op
{
	Allocate,
	Reserve,
	Release,
	Commit,
	Decommit,
	Protect,
	Lock,
	Unlock
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Error
{
	code: ErrorType,
	kind: ErrorKind
}

impl Error
{
	#[inline(always)]
	pub const fn new(kind: ErrorKind, code: ErrorType) -> Self { Self { code, kind } }

	#[inline(always)]
	pub const fn kind(self) -> ErrorKind { self.kind }

	#[inline(always)]
	pub const fn code(self) -> ErrorType { self.code }
}

impl error::Error for Error
{
}

impl fmt::Display for Error
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{} (os error {})", self.kind, self.code)
	}
}
