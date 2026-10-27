// codegen probe: cargo rustc --release --example asm -- --emit asm -C llvm-args=-x86-asm-syntax=intel
// each wrapper should be one os call + test + branch, the error path a jump into last_error

use rngms_memory::{Region, Protection, Result};

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe fn asm_allocate(length: usize) -> Result<Region> { unsafe { Region::allocate(length, Protection::ReadWrite) } }

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe fn asm_map(length: usize) -> Result<Region> { unsafe { Region::reserve(length) } }

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe fn asm_unmap(region: Region, length: usize) -> Result<()> { unsafe { region.release(length) } }

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe fn asm_commit(region: &Region, offset: usize, length: usize) -> Result<()> { unsafe { region.commit(offset, length) } }

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe fn asm_decommit(region: &Region, offset: usize, length: usize) -> Result<()> { unsafe { region.decommit(offset, length) } }

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe fn asm_protect(region: &Region, offset: usize, length: usize, prot: Protection) -> Result<()> { unsafe { region.protect(offset, length, prot) } }

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe fn asm_lock(region: &Region, offset: usize, length: usize) -> Result<()> { unsafe { region.lock(offset, length) } }

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe fn asm_unlock(region: &Region, offset: usize, length: usize) -> Result<()> { unsafe { region.unlock(offset, length) } }

#[inline(never)]
#[unsafe(no_mangle)]
pub fn asm_page_size() -> usize { rngms_memory::page_size() }

fn main()
{
	println!("page size {}", asm_page_size());
}
