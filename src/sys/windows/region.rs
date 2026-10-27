use core::{mem::MaybeUninit, ptr::{self, NonNull}};

use windows_sys::Win32::System::Memory::{
	VirtualAlloc, VirtualFree,
	VirtualProtect,
	VirtualLock, VirtualUnlock,
	MEM_RESERVE, MEM_COMMIT, MEM_RELEASE, MEM_DECOMMIT,
	PAGE_NOACCESS, PAGE_READWRITE
};

use crate::{Region, Result, error::Op};

use super::{ProtectionType, last_error};

#[inline(always)]
pub(crate) unsafe fn allocate(length: usize, prot: ProtectionType) -> Result<NonNull<u8>>
{
	let ptr = unsafe { VirtualAlloc(ptr::null(), length, MEM_RESERVE | MEM_COMMIT, prot) };

	match NonNull::new(ptr.cast())
	{
		Some(ptr) => Ok(ptr),
		None => Err(last_error(Op::Allocate))
	}
}

#[inline(always)]
pub(crate) unsafe fn map(length: usize) -> Result<NonNull<u8>>
{
	let ptr = unsafe { VirtualAlloc(ptr::null(), length, MEM_RESERVE, PAGE_NOACCESS) };

	match NonNull::new(ptr.cast())
	{
		Some(ptr) => Ok(ptr),
		None => Err(last_error(Op::Reserve))
	}
}

#[inline(always)]
pub(crate) unsafe fn unmap(ptr: NonNull<u8>, _length: usize) -> Result<()>
{
	// MEM_RELEASE requires size 0 and frees the whole reservation
	match unsafe { VirtualFree(ptr.as_ptr().cast(), 0, MEM_RELEASE) }
	{
		0 => Err(last_error(Op::Release)),
		_ => Ok(())
	}
}

#[inline(always)]
pub(crate) unsafe fn commit(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	match unsafe { VirtualAlloc(ptr.as_ptr().cast(), length, MEM_COMMIT, PAGE_READWRITE) }.is_null()
	{
		true => Err(last_error(Op::Commit)),
		false => Ok(())
	}
}

#[inline(always)]
pub(crate) unsafe fn decommit(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	match unsafe { VirtualFree(ptr.as_ptr().cast(), length, MEM_DECOMMIT) }
	{
		0 => Err(last_error(Op::Decommit)),
		_ => Ok(())
	}
}

#[inline(always)]
pub(crate) unsafe fn protect(ptr: NonNull<u8>, length: usize, prot: ProtectionType) -> Result<()>
{
	// the old value is mandatory for VirtualProtect, but nobody needs it
	let mut old = MaybeUninit::uninit();

	match unsafe { VirtualProtect(ptr.as_ptr().cast(), length, prot, old.as_mut_ptr()) }
	{
		0 => Err(last_error(Op::Protect)),
		_ => Ok(())
	}
}

#[inline(always)]
pub(crate) unsafe fn lock(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	match unsafe { VirtualLock(ptr.as_ptr().cast(), length) }
	{
		0 => Err(last_error(Op::Lock)),
		_ => Ok(())
	}
}

#[inline(always)]
pub(crate) unsafe fn unlock(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	match unsafe { VirtualUnlock(ptr.as_ptr().cast(), length) }
	{
		0 => Err(last_error(Op::Unlock)),
		_ => Ok(())
	}
}

impl Region
{
	// placeholder
}
