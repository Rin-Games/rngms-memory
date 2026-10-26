use windows_sys::Win32::System::Memory::{
	PAGE_EXECUTE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_NOACCESS, PAGE_READONLY, PAGE_READWRITE,
	PAGE_WRITECOPY, PAGE_EXECUTE_WRITECOPY,
	PAGE_GUARD, PAGE_WRITECOMBINE, PAGE_NOCACHE,
	PAGE_PROTECTION_FLAGS
};

use crate::Protection;

pub type ProtectionType = PAGE_PROTECTION_FLAGS;

pub const NONE: ProtectionType = PAGE_NOACCESS;
pub const READ: ProtectionType = PAGE_READONLY;
pub const READ_WRITE: ProtectionType = PAGE_READWRITE;
pub const EXECUTE: ProtectionType = PAGE_EXECUTE;
pub const READ_EXECUTE: ProtectionType = PAGE_EXECUTE_READ;
pub const READ_WRITE_EXECUTE: ProtectionType = PAGE_EXECUTE_READWRITE;

#[inline(always)]
pub const fn is_readable(p: ProtectionType) -> bool
{
	p & (PAGE_READONLY | PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY) != 0
}

#[inline(always)]
pub const fn is_writable(p: ProtectionType) -> bool
{
	p & (PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY) != 0
}

#[inline(always)]
pub const fn is_executable(p: ProtectionType) -> bool
{
	p & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY) != 0
}

#[allow(non_upper_case_globals)]
impl Protection
{
	pub const WriteCopy: Self = Self(PAGE_WRITECOPY);
	pub const WriteCopyExecute: Self = Self(PAGE_EXECUTE_WRITECOPY);

	#[inline(always)]
	pub const fn guard(self) -> Self { Self(self.0 | PAGE_GUARD) }

	#[inline(always)]
	pub const fn write_combine(self) -> Self { Self(self.0 | PAGE_WRITECOMBINE) }

	#[inline(always)]
	pub const fn no_cache(self) -> Self { Self(self.0 | PAGE_NOCACHE) }
}
