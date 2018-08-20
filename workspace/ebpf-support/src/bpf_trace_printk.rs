// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// int bpf_trace_printk(const char *fmt, u32 fmt_size, ...)
#[macro_export]
macro_rules! bpf_trace_printk
{
	($format: expr, $fmt_size: expr) =>
	{
		{
			use $crate::libc::*;
			use $crate::bpf_func_id::*;
			
			let function_pointer: extern "C" fn(*const c_char, fmt_size: u32, ...) -> c_int = unsafe { ::std::mem::transmute(BPF_FUNC_trace_printk) };
			function_pointer($format, $fmt_size)
		}
	}
}
