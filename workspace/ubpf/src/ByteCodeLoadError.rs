// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// A byte code load error.
#[derive(Debug)]
pub enum ByteCodeLoadError
{
	/// An error loading byte code.
	Input(io::Error),
	
	/// A byte code parsing error.
	Parse(String),
}

impl Display for ByteCodeLoadError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		use self::ByteCodeLoadError::*;
		
		match *self
		{
			Input(ref error) => write!(f, "could not load bytecode because {}", error),
			
			Parse(ref string) => write!(f, "parse error '{}'", string),
		}
	}
}

impl Error for ByteCodeLoadError
{
	#[inline(always)]
	fn cause(&self) -> Option<&Error>
	{
		use self::ByteCodeLoadError::*;
		
		match *self
		{
			Input(ref error) => Some(error),
			
			Parse(_) => None,
		}
	}
}

impl From<io::Error> for ByteCodeLoadError
{
	fn from(error: io::Error) -> Self
	{
		ByteCodeLoadError::Input(error)
	}
}

impl ByteCodeLoadError
{
	#[inline(always)]
	pub(crate) fn error_message_to_string(error_message: *mut c_char) -> String
	{
		debug_assert!(!error_message.is_null(), "error message is null");
		
		let error_message_string = (unsafe { CStr::from_ptr(error_message) }).to_string_lossy().into_owned();
		
		unsafe { free(error_message as *mut _) };
		
		error_message_string
	}
	
	#[inline(always)]
	pub(crate) fn load_error(error_message: *mut c_char) -> Result<VirtualMachineWithByteCode, ByteCodeLoadError>
	{
		Err(ByteCodeLoadError::Parse(Self::error_message_to_string(error_message)))
	}
}
