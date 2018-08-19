// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Represents a virtual machine with loaded, parsed and compiled byte code and registered external functions.
#[derive(Debug, PartialEq, Eq)]
pub struct VirtualMachineWithCompiledByteCode
{
	function_pointer: unsafe extern "C" fn(mem: *mut c_void, mem_len: usize) -> u64,
	keep_alive: VirtualMachineWithByteCode,
}

impl Execute for VirtualMachineWithCompiledByteCode
{
	#[inline(always)]
	fn execute(&self, memory: &mut [u8]) -> Result<u64, ()>
	{
		let result = unsafe { (self.function_pointer)(memory.as_mut_ptr() as *mut _, memory.len()) };
		if unlikely!(result == ::std::u64::MAX)
		{
			Err(())
		}
		else
		{
			Ok(result)
		}
	}
}
