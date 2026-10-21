use windows_sys::Win32::System::Memory::{
	PAGE_EXECUTE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_NOACCESS, PAGE_READONLY, PAGE_READWRITE,
	PAGE_WRITECOPY, PAGE_EXECUTE_WRITECOPY,
	PAGE_GUARD, PAGE_WRITECOMBINE, PAGE_NOCACHE,
	PAGE_PROTECTION_FLAGS
};

use crate::Protection;

pub type ProtectionType = PAGE_PROTECTION_FLAGS;

#[allow(non_upper_case_globals)]
impl Protection
{
	// on all platforms
	pub const None: Self = Self(PAGE_NOACCESS);
	pub const Read: Self = Self(PAGE_READONLY);
	pub const ReadWrite: Self = Self(PAGE_READWRITE);
	pub const Execute: Self = Self(PAGE_EXECUTE);
	pub const ReadExecute: Self = Self(PAGE_EXECUTE_READ);
	pub const ReadWriteExecute: Self = Self(PAGE_EXECUTE_READWRITE);

	#[inline(always)]
	pub const fn is_readable(self) -> bool
	{
		self.0 & (PAGE_READONLY | PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY) != 0
	}

	#[inline(always)]
	pub const fn is_writable(self) -> bool
	{
		self.0 & (PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY) != 0
	}

	#[inline(always)]
	pub const fn is_executable(self) -> bool
	{
		self.0 & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY) != 0
	}

	// windows only
	pub const WriteCopy: Self = Self(PAGE_WRITECOPY);
	pub const WriteCopyExecute: Self = Self(PAGE_EXECUTE_WRITECOPY);

	#[inline(always)]
	pub const fn guard(self) -> Self { Self(self.0 | PAGE_GUARD) }

	#[inline(always)]
	pub const fn write_combine(self) -> Self { Self(self.0 | PAGE_WRITECOMBINE) }

	#[inline(always)]
	pub const fn no_cache(self) -> Self { Self(self.0 | PAGE_NOCACHE) }
}
