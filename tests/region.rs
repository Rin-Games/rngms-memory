use rngms_memory::{Region, Protection, ErrorKind, page_size, allocation_granularity};

#[test]
fn page_size_is_sane()
{
	let page = page_size();
	let granularity = allocation_granularity();

	assert!(page.is_power_of_two());
	assert!(granularity.is_power_of_two());
	assert_eq!(granularity % page, 0);

	// repeated query returns the same values
	assert_eq!(page_size(), page);
	assert_eq!(allocation_granularity(), granularity);
}

#[test]
fn reserve_commit_decommit_release()
{
	let page = page_size();
	let length = allocation_granularity() * 4;

	unsafe
	{
		let region = Region::reserve(length).unwrap();
		assert_eq!(region.as_ptr() as usize % allocation_granularity(), 0);

		region.commit(page, page * 2).unwrap();

		let ptr = region.add(page).as_ptr();
		ptr.write_bytes(0xAB, page * 2);
		assert_eq!(*ptr, 0xAB);
		assert_eq!(*ptr.add(page * 2 - 1), 0xAB);

		region.decommit(page, page * 2).unwrap();

		// recommitted pages come back zeroed
		region.commit(page, page).unwrap();
		assert_eq!(*ptr, 0);

		region.release(length).unwrap();
	}
}

#[test]
fn allocate_read_write()
{
	let length = page_size() * 2;

	unsafe
	{
		let region = Region::allocate(length, Protection::ReadWrite).unwrap();

		region.as_ptr().write_bytes(0x5A, length);
		assert_eq!(*region.as_ptr().add(length - 1), 0x5A);

		region.protect(0, length, Protection::Read).unwrap();
		assert_eq!(*region.as_ptr(), 0x5A);

		region.release(length).unwrap();
	}
}

#[test]
fn lock_unlock()
{
	let length = page_size();

	unsafe
	{
		let region = Region::allocate(length, Protection::ReadWrite).unwrap();

		// locking may hit RLIMIT_MEMLOCK on unix, only check that unlock pairs with a successful lock
		if region.lock(0, length).is_ok()
		{
			region.unlock(0, length).unwrap();
		}

		region.release(length).unwrap();
	}
}

#[test]
fn protection_flags()
{
	assert!(!Protection::None.is_readable());
	assert!(Protection::Read.is_readable() && !Protection::Read.is_writable());
	assert!(Protection::ReadWrite.is_writable() && !Protection::ReadWrite.is_executable());
	assert!(Protection::ReadExecute.is_executable() && !Protection::ReadExecute.is_writable());
	assert!(Protection::ReadWriteExecute.is_readable() && Protection::ReadWriteExecute.is_writable() && Protection::ReadWriteExecute.is_executable());
}

#[test]
fn zero_length_is_invalid_argument()
{
	let error = unsafe { Region::reserve(0) }.unwrap_err();
	assert_eq!(error.kind(), ErrorKind::InvalidArgument, "{error}");

	let error = unsafe { Region::allocate(0, Protection::ReadWrite) }.unwrap_err();
	assert_eq!(error.kind(), ErrorKind::InvalidArgument, "{error}");
}
