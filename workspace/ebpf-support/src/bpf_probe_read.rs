// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Safely attempts to read (derefeence) `size` bytes from the address `src` and store the data in `dst`, where `dst` is a reference to a variable on the BPF program's stack.
///
/// Allows program verification of memory accesses and of dereferencing pointers.
///
/// Returns zero (0) on success and a negative value on failure.
///
/// `void bpf_probe_read(void * dst, u32 size, const void * src)`
#[macro_export]
macro_rules! bpf_probe_read
{
	($dst: expr, $size: expr, $src: expr) =>
	{
		{
			const function_identifier: bpf_func_id = $crate::bpf_func_id::probe_read;
			
			use $crate::libc::*;
			
			let function_pointer: extern "C" fn(*mut u8, u32, *const u8) -> c_int = unsafe { ::std::mem::transmute(function_identifier) };
			function_pointer($dst, $size, $src)
		}
	}
}
