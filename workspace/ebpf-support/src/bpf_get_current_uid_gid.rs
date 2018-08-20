// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Gets the current process's user identifier (uid) in the lower 32 bits and group identifier (gid) in the upper 32 bits.
///
/// To get the user identifier: `bpf_get_current_uid_gid!() >> 32`.
///
/// To get the group identifier: `bpf_get_current_uid_gid!() & 0xFFFFFFFF`.
///
/// `u64 bpf_get_current_uid_gid(void)`
#[macro_export]
macro_rules! bpf_get_current_uid_gid
{
	() =>
	{
		{
			const function_identifier: bpf_func_id = $crate::bpf_func_id::get_current_uid_gid;
			
			let function_pointer: extern "C" fn() -> u64 = unsafe { ::std::mem::transmute(function_identifier) };
			function_pointer()
		}
	}
}
