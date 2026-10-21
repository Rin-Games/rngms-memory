use core::fmt;

use crate::sys;

pub use sys::ProtectionType;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Protection(pub(crate) ProtectionType);

impl Protection
{
	#[inline(always)]
	pub const fn to_raw(self) -> ProtectionType { self.0 }

	#[inline(always)]
	pub const fn from_raw(raw: ProtectionType) -> Self { Self(raw) }
}

impl fmt::Display for Protection
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(
			f, "{}{}{}",
			if self.is_readable() { 'r' } else { '-' },
			if self.is_writable() { 'w' } else { '-' },
			if self.is_executable() { 'x' } else { '-' },
		)
	}
}

impl fmt::Debug for Protection
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "Protection({} {:#x})", self, self.to_raw())
	}
}
