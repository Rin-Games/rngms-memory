use libc::{c_int, ENOMEM, EAGAIN, EPERM, EACCES, EINVAL, ENOTSUP, EOPNOTSUPP};

use crate::{Error, ErrorKind, error::Op};

pub type ErrorType = c_int;

#[inline(always)]
unsafe fn errno() -> c_int
{
	#[cfg(any(target_os = "linux", target_os = "emscripten", target_os = "hurd", target_os = "dragonfly"))]
	use libc::__errno_location as location;

	#[cfg(any(target_vendor = "apple", target_os = "freebsd"))]
	use libc::__error as location;

	#[cfg(any(target_os = "android", target_os = "netbsd", target_os = "openbsd", target_os = "cygwin"))]
	use libc::__errno as location;

	#[cfg(any(target_os = "illumos", target_os = "solaris"))]
	use libc::___errno as location;

	#[cfg(target_os = "haiku")]
	use libc::_errnop as location;

	#[cfg(target_os = "aix")]
	use libc::_Errno as location;

	#[cfg(not(any(
		target_os = "linux", target_os = "emscripten", target_os = "hurd", target_os = "dragonfly",
		target_vendor = "apple", target_os = "freebsd",
		target_os = "android", target_os = "netbsd", target_os = "openbsd", target_os = "cygwin",
		target_os = "illumos", target_os = "solaris",
		target_os = "haiku",
		target_os = "aix"
	)))]
	compile_error!("rngms-memory: errno is not wired for this unix target");

	unsafe { *location() }
}

// kept out of line so the success path stays a call + test + branch
#[cold]
#[inline(never)]
pub(crate) fn last_error(op: Op) -> Error
{
	let code = unsafe { errno() };

	let kind = match code
	{
		ENOMEM => match op
		{
			Op::Reserve => ErrorKind::OutOfAddressSpace,
			Op::Allocate | Op::Commit | Op::Decommit => ErrorKind::OutOfMemory,
			Op::Lock => ErrorKind::LockLimit,
			// range is not mapped
			Op::Release | Op::Protect | Op::Unlock => ErrorKind::InvalidAddress
		},
		EAGAIN => match op
		{
			Op::Lock => ErrorKind::LockLimit,
			_ => ErrorKind::OutOfMemory
		},
		EPERM => match op
		{
			Op::Lock => ErrorKind::LockLimit,
			_ => ErrorKind::PermissionDenied
		},
		EACCES => ErrorKind::PermissionDenied,
		EINVAL => ErrorKind::InvalidArgument,
		#[allow(unreachable_patterns)] // ENOTSUP == EOPNOTSUPP on linux
		ENOTSUP | EOPNOTSUPP => ErrorKind::Unsupported,
		_ => ErrorKind::Other
	};

	Error::new(kind, code)
}
