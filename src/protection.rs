use core::fmt;

use crate::sys;

pub use sys::ProtectionType;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Protection(pub(crate) ProtectionType);

#[allow(non_upper_case_globals)]
impl Protection
{
	pub const None: Self = Self(sys::NONE);
	pub const Read: Self = Self(sys::READ);
	pub const ReadWrite: Self = Self(sys::READ_WRITE);
	pub const Execute: Self = Self(sys::EXECUTE);
	pub const ReadExecute: Self = Self(sys::READ_EXECUTE);
	pub const ReadWriteExecute: Self = Self(sys::READ_WRITE_EXECUTE);

	#[inline(always)]
	pub const fn to_raw(self) -> ProtectionType { self.0 }

	#[inline(always)]
	pub const fn from_raw(raw: ProtectionType) -> Self { Self(raw) }

	#[inline(always)]
	pub const fn is_readable(self) -> bool { sys::is_readable(self.0) }

	#[inline(always)]
	pub const fn is_writable(self) -> bool { sys::is_writable(self.0) }

	#[inline(always)]
	pub const fn is_executable(self) -> bool { sys::is_executable(self.0) }
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
