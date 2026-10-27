use core::ptr::{NonNull};

use crate::{Protection, Result, sys};

#[repr(transparent)]
#[derive(PartialEq, Eq, Debug)]
pub struct Region(pub(crate) NonNull<u8>);

impl Region
{
	// allocation

	#[inline(always)]
	pub unsafe fn allocate(length: usize, prot: Protection) -> Result<Self>
	{
		unsafe { sys::allocate(length, prot.0) }.map(Self)
	}

	#[inline(always)]
	pub unsafe fn reserve(length: usize) -> Result<Self>
	{
		unsafe { sys::map(length) }.map(Self)
	}

	#[inline(always)]
	pub unsafe fn release(self, length: usize) -> Result<()>
	{
		unsafe { sys::unmap(self.0, length) }
	}

	// access

	#[inline(always)]
	pub const fn as_ptr(&self) -> *mut u8 { self.0.as_ptr() }

	#[inline(always)]
	pub const fn as_non_null(&self) -> NonNull<u8> { self.0 }

	#[inline(always)]
	pub const unsafe fn add(&self, offset: usize) -> NonNull<u8> { unsafe {self.0.add(offset)} }

	// pages

	#[inline(always)]
	pub unsafe fn commit(&self, offset: usize, length: usize) -> Result<()>
	{
		unsafe { sys::commit(self.0.add(offset), length) }
	}

	#[inline(always)]
	pub unsafe fn decommit(&self, offset: usize, length: usize) -> Result<()>
	{
		unsafe { sys::decommit(self.0.add(offset), length) }
	}

	#[inline(always)]
	pub unsafe fn protect(&self, offset: usize, length: usize, prot: Protection) -> Result<()>
	{
		unsafe { sys::protect(self.0.add(offset), length, prot.0) }
	}

	// working set

	#[inline(always)]
	pub unsafe fn lock(&self, offset: usize, length: usize) -> Result<()>
	{
		unsafe { sys::lock(self.0.add(offset), length) }
	}

	#[inline(always)]
	pub unsafe fn unlock(&self, offset: usize, length: usize) -> Result<()>
	{
		unsafe { sys::unlock(self.0.add(offset), length) }
	}
}
