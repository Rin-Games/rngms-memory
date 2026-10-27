mod protection;
mod error;
mod region;

pub use protection::*;
pub use error::*;
pub(crate) use region::*;

use core::mem::MaybeUninit;

use windows_sys::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO};

// plain queries, no caching: the caller keeps the values

#[inline(always)]
pub fn page_size() -> usize
{
	system_info().dwPageSize as usize
}

// reservations start on this boundary (64K), not on the page size
#[inline(always)]
pub fn allocation_granularity() -> usize
{
	system_info().dwAllocationGranularity as usize
}

#[inline(always)]
fn system_info() -> SYSTEM_INFO
{
	let mut info = MaybeUninit::uninit();

	unsafe
	{
		GetSystemInfo(info.as_mut_ptr());
		info.assume_init()
	}
}
