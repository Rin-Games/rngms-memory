use core::ptr::{self, NonNull};

use libc::{
	mmap, munmap,
	mprotect,
	mlock, munlock,
	MAP_PRIVATE, MAP_ANONYMOUS, MAP_FIXED, MAP_FAILED,
	PROT_NONE, PROT_READ, PROT_WRITE
};

use crate::{Result, error::Op};

use super::{ProtectionType, last_error};

// PROT_NONE private anonymous memory is not charged against the commit limit,
// the charge happens when pages become writable
const FLAGS: i32 = MAP_PRIVATE | MAP_ANONYMOUS;

#[inline(always)]
pub(crate) unsafe fn allocate(length: usize, prot: ProtectionType) -> Result<NonNull<u8>>
{
	let ptr = unsafe { mmap(ptr::null_mut(), length, prot, FLAGS, -1, 0) };

	match ptr == MAP_FAILED
	{
		// mmap never returns null without MAP_FIXED
		false => Ok(unsafe { NonNull::new_unchecked(ptr.cast()) }),
		true => Err(last_error(Op::Allocate))
	}
}

#[inline(always)]
pub(crate) unsafe fn map(length: usize) -> Result<NonNull<u8>>
{
	let ptr = unsafe { mmap(ptr::null_mut(), length, PROT_NONE, FLAGS, -1, 0) };

	match ptr == MAP_FAILED
	{
		// mmap never returns null without MAP_FIXED
		false => Ok(unsafe { NonNull::new_unchecked(ptr.cast()) }),
		true => Err(last_error(Op::Reserve))
	}
}

#[inline(always)]
pub(crate) unsafe fn unmap(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	match unsafe { munmap(ptr.as_ptr().cast(), length) }
	{
		0 => Ok(()),
		_ => Err(last_error(Op::Release))
	}
}

#[inline(always)]
pub(crate) unsafe fn commit(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	match unsafe { mprotect(ptr.as_ptr().cast(), length, PROT_READ | PROT_WRITE) }
	{
		0 => Ok(()),
		_ => Err(last_error(Op::Commit))
	}
}

#[inline(always)]
pub(crate) unsafe fn decommit(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	// remapping over the range drops the pages and the commit charge in one syscall
	let res = unsafe { mmap(ptr.as_ptr().cast(), length, PROT_NONE, FLAGS | MAP_FIXED, -1, 0) };

	match res == MAP_FAILED
	{
		false => Ok(()),
		true => Err(last_error(Op::Decommit))
	}
}

#[inline(always)]
pub(crate) unsafe fn protect(ptr: NonNull<u8>, length: usize, prot: ProtectionType) -> Result<()>
{
	match unsafe { mprotect(ptr.as_ptr().cast(), length, prot) }
	{
		0 => Ok(()),
		_ => Err(last_error(Op::Protect))
	}
}

#[inline(always)]
pub(crate) unsafe fn lock(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	match unsafe { mlock(ptr.as_ptr().cast(), length) }
	{
		0 => Ok(()),
		_ => Err(last_error(Op::Lock))
	}
}

#[inline(always)]
pub(crate) unsafe fn unlock(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	match unsafe { munlock(ptr.as_ptr().cast(), length) }
	{
		0 => Ok(()),
		_ => Err(last_error(Op::Unlock))
	}
}
