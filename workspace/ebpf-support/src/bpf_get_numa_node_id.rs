// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Gets the NUMA node identifier of the processor running the program.
///
/// NUMA nodes are typically from 0 to 7 inclusive.
///
/// ?Some Linux systems with NUMA disabled may return a negative number?
///
/// Consider using a processor assembler instruction in preference.
///
/// See also `bpf_get_smp_processor_id!()`.
///
/// `i32 bpf_get_numa_node_id(void)`
#[macro_export]
macro_rules! bpf_get_numa_node_id
{
	() =>
	{
		{
			const function_identifier: bpf_func_id = $crate::bpf_func_id::get_numa_node_id;
			
			let function_pointer: extern "C" fn() -> i32 = unsafe { ::std::mem::transmute(function_identifier) };
			function_pointer()
		}
	}
}
