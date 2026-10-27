use libc::{c_int, PROT_NONE, PROT_READ, PROT_WRITE, PROT_EXEC};

pub type ProtectionType = c_int;

pub const NONE: ProtectionType = PROT_NONE;
pub const READ: ProtectionType = PROT_READ;
pub const READ_WRITE: ProtectionType = PROT_READ | PROT_WRITE;
pub const EXECUTE: ProtectionType = PROT_EXEC;
pub const READ_EXECUTE: ProtectionType = PROT_READ | PROT_EXEC;
pub const READ_WRITE_EXECUTE: ProtectionType = PROT_READ | PROT_WRITE | PROT_EXEC;

#[inline(always)]
pub const fn is_readable(p: ProtectionType) -> bool
{
	p & PROT_READ != 0
}

#[inline(always)]
pub const fn is_writable(p: ProtectionType) -> bool
{
	p & PROT_WRITE != 0
}

#[inline(always)]
pub const fn is_executable(p: ProtectionType) -> bool
{
	p & PROT_EXEC != 0
}
