use core::ptr::NonNull;

use windows_sys::Win32::System::Memory::{
	VirtualProtect,
	VirtualLock, VirtualUnlock
};

use crate::{ProtectionType, Region, Result};

#[inline(always)]
pub unsafe fn map(length: usize) -> Result<NonNull<u8>>
{
}

#[inline(always)]
pub unsafe fn unmap(ptr: NonNull<u8>, length: usize) -> Result<()>
{
}

#[inline(always)]
pub unsafe fn commit(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	Ok(())
}

#[inline(always)]
pub unsafe fn decommit(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	Ok(())
}

#[inline(always)]
pub unsafe fn protect(ptr: NonNull<u8>, length: usize, prot: ProtectionType) -> Result<()>
{
	Ok(())
}

#[inline(always)]
pub unsafe fn lock(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	if unsafe {VirtualLock(ptr.as_ptr().cast(), length)} == 0
	{
		Err(())
	}
	Ok(())
}

#[inline(always)]
pub unsafe fn unlock(ptr: NonNull<u8>, length: usize) -> Result<()>
{
}


impl Region
{
}
