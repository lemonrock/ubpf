// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Returns the time elapsed since system boot, in nanoseconds.
///
/// `u64 bpf_ktime_get_ns(void)`
#[macro_export]
macro_rules! bpf_ktime_get_ns
{
	() =>
	{
		{
			const function_identifier: bpf_func_id = $crate::bpf_func_id::ktime_get_ns;
			
			let function_pointer: extern "C" fn() -> u64 = unsafe { ::std::mem::transmute(function_identifier) };
			function_pointer()
		}
	}
}
