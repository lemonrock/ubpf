// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Gets a pseudo-random 32-bit unsigned integer.
///
/// From a security point of view, this helper uses its ownpseudo-random internal state, and cannot be used to infer the seed of other random functions in the kernel.
///
/// However, it is essential to note that the generator used by the helper is not cryptographically secure.
///
/// Consider using a processor assembler instruction in preference to this function.
///
/// `u32 bpf_get_prandom_u32(void)`
#[macro_export]
macro_rules! bpf_get_prandom_u32
{
	() =>
	{
		{
			const function_identifier: bpf_func_id = $crate::bpf_func_id::get_prandom_u32;
			
			let function_pointer: extern "C" fn() -> u32 = unsafe { ::std::mem::transmute(function_identifier) };
			function_pointer()
		}
	}
}
