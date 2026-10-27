// every platform exports the same set of symbols, no trait in between:
//
// types      ProtectionType, ErrorType
// constants  NONE, READ, READ_WRITE, EXECUTE, READ_EXECUTE, READ_WRITE_EXECUTE
// const fn   is_readable, is_writable, is_executable
// unsafe fn  allocate, map, unmap, commit, decommit, protect, lock, unlock
// fn         page_size, allocation_granularity
// fn         last_error (cold)
//
// platform extras go into inherent impls on Region / Protection inside the platform module

#[cfg(windows)] mod windows;
#[cfg(windows)] pub use windows::*;

#[cfg(unix)] mod unix;
#[cfg(unix)] pub use unix::*;

#[cfg(not(any(windows, unix)))] compile_error!("rngms-memory: unsupported platform");

// signature checks, a platform that drifts from the contract fails to compile here
mod contract
{
	use core::ptr::NonNull;

	use crate::{Error, Result, error::Op};

	use super::*;

	const _: Option<ErrorType> = None;

	const _: ProtectionType = NONE;
	const _: ProtectionType = READ;
	const _: ProtectionType = READ_WRITE;
	const _: ProtectionType = EXECUTE;
	const _: ProtectionType = READ_EXECUTE;
	const _: ProtectionType = READ_WRITE_EXECUTE;

	const _: fn(ProtectionType) -> bool = is_readable;
	const _: fn(ProtectionType) -> bool = is_writable;
	const _: fn(ProtectionType) -> bool = is_executable;

	const _: unsafe fn(usize, ProtectionType) -> Result<NonNull<u8>> = allocate;
	const _: unsafe fn(usize) -> Result<NonNull<u8>> = map;
	const _: unsafe fn(NonNull<u8>, usize) -> Result<()> = unmap;
	const _: unsafe fn(NonNull<u8>, usize) -> Result<()> = commit;
	const _: unsafe fn(NonNull<u8>, usize) -> Result<()> = decommit;
	const _: unsafe fn(NonNull<u8>, usize, ProtectionType) -> Result<()> = protect;
	const _: unsafe fn(NonNull<u8>, usize) -> Result<()> = lock;
	const _: unsafe fn(NonNull<u8>, usize) -> Result<()> = unlock;

	const _: fn() -> usize = page_size;
	const _: fn() -> usize = allocation_granularity;

	const _: fn(Op) -> Error = last_error;
}
