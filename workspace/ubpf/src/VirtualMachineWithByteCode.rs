// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Represents a virtual machine with loaded and parsed byte code and registered external functions.
#[derive(Debug, PartialEq, Eq)]
pub struct VirtualMachineWithByteCode(VirtualMachine);

impl Execute for VirtualMachineWithByteCode
{
	#[inline(always)]
	fn execute(&self, memory: &mut [u8]) -> Result<u64, ()>
	{
		let result = unsafe { ubpf_exec(self.as_ptr(), memory.as_mut_ptr() as *mut _, memory.len()) };
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

impl VirtualMachineWithByteCode
{
	/// Compile; only possible for x86-64.
	#[cfg(target_arch = "x86_64")]
	#[inline(always)]
	pub fn compile(self) -> Result<VirtualMachineWithCompiledByteCode, ByteCodeCompileError>
	{
		let mut error_message = null_mut();
		
		let function_pointer = unsafe { ubpf_compile(self.as_ptr(), &mut error_message) };
		match function_pointer
		{
			None => Err(ByteCodeCompileError::Compile(ByteCodeLoadError::error_message_to_string(error_message))),
			Some(function_pointer) => Ok
			(
				VirtualMachineWithCompiledByteCode
				{
					function_pointer,
					keep_alive: self,
				}
			)
		}
	}
	
	#[inline(always)]
	fn as_ptr(&self) -> *mut ubpf_vm
	{
		self.0.as_ptr()
	}
}
