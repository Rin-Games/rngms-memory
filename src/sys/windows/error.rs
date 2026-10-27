use windows_sys::Win32::Foundation::{
	GetLastError,
	ERROR_NOT_ENOUGH_MEMORY, ERROR_OUTOFMEMORY, ERROR_COMMITMENT_LIMIT,
	ERROR_WORKING_SET_QUOTA,
	ERROR_ACCESS_DENIED, ERROR_DYNAMIC_CODE_BLOCKED,
	ERROR_INVALID_ADDRESS, ERROR_NOACCESS, ERROR_INVALID_PARAMETER,
	ERROR_PRIVILEGE_NOT_HELD,
	WIN32_ERROR
};

use crate::{Error, ErrorKind, error::Op};

pub type ErrorType = WIN32_ERROR;

// kept out of line so the success path stays a call + test + branch
#[cold]
#[inline(never)]
pub(crate) fn last_error(op: Op) -> Error
{
	let code = unsafe { GetLastError() };

	let kind = match code
	{
		ERROR_NOT_ENOUGH_MEMORY | ERROR_OUTOFMEMORY | ERROR_COMMITMENT_LIMIT => match op
		{
			Op::Reserve => ErrorKind::OutOfAddressSpace,
			Op::Lock => ErrorKind::LockLimit,
			_ => ErrorKind::OutOfMemory
		},
		ERROR_WORKING_SET_QUOTA => ErrorKind::LockLimit,
		ERROR_ACCESS_DENIED | ERROR_DYNAMIC_CODE_BLOCKED => ErrorKind::PermissionDenied,
		ERROR_INVALID_ADDRESS | ERROR_NOACCESS => ErrorKind::InvalidAddress,
		ERROR_INVALID_PARAMETER => ErrorKind::InvalidArgument,
		ERROR_PRIVILEGE_NOT_HELD => ErrorKind::Unsupported,
		_ => ErrorKind::Other
	};

	Error::new(kind, code)
}
