// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// An ubpf Virtual Machine.
#[derive(Debug, PartialEq, Eq)]
pub struct VirtualMachine
{
	handle: NonNull<ubpf_vm>,
	registered_external_function_names: HashSet<CString>,
}

impl Drop for VirtualMachine
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ubpf_destroy(self.as_ptr()) }
	}
}

impl Default for VirtualMachine
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new().expect("out of memory")
	}
}

impl VirtualMachine
{
	/// New instance.
	///
	/// Returns an error in the event that C's malloc functions are out-of-memory.
	#[inline(always)]
	pub fn new() -> Result<Self, ()>
	{
		let result = unsafe { ubpf_create() };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok
			(
				VirtualMachine
				{
					handle: unsafe { NonNull::new_unchecked(result) },
					registered_external_function_names: Default::default(),
				}
			)
		}
	}
	
	/// Registers external function names, assigning each (internally) to an index.
	///
	/// Up to 2³² - 1 functions are supported.
	#[inline(always)]
	pub fn register_external_function(&mut self, name: CString, function_pointer: NonNull<c_void>) -> Result<(), ()>
	{
		debug_assert!(self.registered_external_function_names.contains(&name), "'{:?}' has already been registered", name);
		
		let potential_index = self.registered_external_function_names.len();
		debug_assert!(potential_index < ::std::u32::MAX as usize, "Can not register more than ::std::u32::MAX - 1 functions");
		let index = potential_index as u32;
		
		let result = unsafe { ubpf_register(self.as_ptr(), index, name.as_ptr(), function_pointer.as_ptr()) };
		if likely!(result == 0)
		{
			self.registered_external_function_names.insert(name);
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(())
		}
		else
		{
			panic!("Unexpected result '{}' from ubpf_register", result)
		}
	}
	
	/*
	 * Load code into a VM
	 *
	 * This must be done before calling ubpf_exec or ubpf_compile and after
	 * registering all functions.
	 *
	 * 'code' should point to eBPF bytecodes and 'code_len' should be the size in
	 * bytes of that buffer.
	 *
	 * Returns 0 on success, -1 on error. In case of error a pointer to the error
	 * message will be stored in 'errmsg' and should be freed by the caller.
	 */
	/// Loads code.
	///
	/// After doing this, external functions can no longer be registered.
	///
	/// Bytecode with a `.len()` up to 2³² - 1 inclusive is supported.
	#[inline(always)]
	pub fn load_from_eBPF_bytecode<'a>(self, eBPF_bytecode: impl ByteSource<'a>) -> Result<VirtualMachineWithByteCode, ByteCodeLoadError>
	{
		let bytes = eBPF_bytecode.bytes()?;
		let bytecode: &[u8] = bytes.borrow();
		
		let potential_bytecode_length = bytecode.len();
		debug_assert_eq!(potential_bytecode_length % 8, 0, "eBPF_bytecode length must be a multiple of 8; length was '{}'", potential_bytecode_length);
		debug_assert!(potential_bytecode_length < ::std::u32::MAX as usize, "Can not load more than ::std::u32::MAX - 1 bytecode bytes");
		let bytecode_length = potential_bytecode_length as u32;
		
		let mut error_message = null_mut();
		
		let result = unsafe { ubpf_load(self.as_ptr(), bytecode.as_ptr() as *const _, bytecode_length, &mut error_message) };
		
		if likely!(result == 0)
		{
			debug_assert!(error_message.is_null(), "error message is not null");
			Ok(VirtualMachineWithByteCode(self))
		}
		else if likely!(result == -1)
		{
			ByteCodeLoadError::load_error(error_message)
		}
		else
		{
			panic!("Unexpected result '{}' from ubpf_load", result)
		}
	}
	
	
	/// Load from the bytes of an ELF file which is:-
	///
	/// * little-endian.
	/// * 64-bit.
	/// * with a single text section containing eBPF bytecodes.
	/// * Version 1 (they all are).
	/// * Has an OS ABI of 'NONE'.
	/// * Is relocatable.
	/// * Has a machine of `NONE` or `BPF`.
	///
	/// This is the format produced by Clang.
	#[inline(always)]
	pub fn load_from_elf_as_produced_by_clang<'a>(self, ELF_bytes: impl ByteSource<'a>) -> Result<VirtualMachineWithByteCode, ByteCodeLoadError>
	{
		let bytes = ELF_bytes.bytes()?;
		let elf: &[u8] = bytes.borrow();
		
		let mut error_message = null_mut();
		
		let result = unsafe { ubpf_load_elf(self.as_ptr(), elf.as_ptr() as *const _, elf.len(), &mut error_message) };
		
		if likely!(result == 0)
		{
			debug_assert!(error_message.is_null(), "error message is not null");
			Ok(VirtualMachineWithByteCode(self))
		}
		else if likely!(result == -1)
		{
			ByteCodeLoadError::load_error(error_message)
		}
		else
		{
			panic!("Unexpected result '{}' from ubpf_load", result)
		}
	}
	
	#[inline(always)]
	pub(crate) fn as_ptr(&self) -> *mut ubpf_vm
	{
		self.handle.as_ptr()
	}
}
