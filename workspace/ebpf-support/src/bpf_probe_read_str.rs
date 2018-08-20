// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Copy a NUL terminated string from an unsafe address `unsafe_ptr` to `dst`.
///
/// The `size` should include the terminating NUL byte.
///
/// In case the string length is smaller than `size`, the target is not padded with further NUL bytes.
/// If the string length is larger than `size`, just `size - 1` bytes are copied and the last byte is set to NUL.
///
/// On success, the a positive length of the copied string is returned, including the trailing NUL character.
///
/// On error, a negative is returned value.
///
/// Returns zero (0) on success and a negative value on failure.
///
/// `void bpf_probe_read_str(void * dst, u32 size, const void * unsafe_ptr)`
#[macro_export]
macro_rules! bpf_probe_read_str
{
	($dst: expr, $size: expr, $unsafe_ptr: expr) =>
	{
		{
			const function_identifier: bpf_func_id = $crate::bpf_func_id::probe_read_str;
			
			use $crate::libc::*;
			
			let function_pointer: extern "C" fn(*mut u8, u32, *const u8) -> c_int = unsafe { ::std::mem::transmute(function_identifier) };
			function_pointer($dst, $size, $unsafe_ptr)
		}
	}
}
