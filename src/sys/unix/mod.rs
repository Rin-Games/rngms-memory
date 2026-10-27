mod protection;
mod error;
mod region;

pub use protection::*;
pub use error::*;
pub(crate) use region::*;

use libc::{sysconf, _SC_PAGESIZE};

// plain queries, no caching: the caller keeps the values

// not a constant, aarch64 linux runs with 4K, 16K or 64K pages
#[inline(always)]
pub fn page_size() -> usize
{
	unsafe { sysconf(_SC_PAGESIZE) as usize }
}

// mmap places mappings on any page boundary
#[inline(always)]
pub fn allocation_granularity() -> usize
{
	page_size()
}
