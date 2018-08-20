// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Copy the name of the executable (excluding the path) (`comm` attribute) of the current task into `buf` of `size_of_buf`.
///
/// Returns zero (0) on success and a negative value on failure.
///
/// On success, `buf` is NUL-terminated.
///
/// On failure, it is zeroed.
///
/// `void bpf_get_current_comm(char * buf, u32 size_of_buf)`
#[macro_export]
macro_rules! bpf_get_current_comm
{
	($buf: expr, $size_of_buf: expr) =>
	{
		{
			const function_identifier: bpf_func_id = $crate::bpf_func_id::get_current_comm;
			
			use $crate::libc::*;
			
			let function_pointer: extern "C" fn(*mut c_char, u32) -> c_int = unsafe { ::std::mem::transmute(function_identifier) };
			function_pointer($buf, $size_of_buf)
		}
	}
}
