use core::ptr::NonNull;

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
	Ok(())
}

#[inline(always)]
pub unsafe fn unlock(ptr: NonNull<u8>, length: usize) -> Result<()>
{
	Ok(())
}


impl Region
{
}
