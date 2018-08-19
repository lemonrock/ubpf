// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// A byte code compile error.
#[derive(Debug)]
pub enum ByteCodeCompileError
{
	/// A byte code compile error.
	Compile(String),
}

impl Display for ByteCodeCompileError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		use self::ByteCodeCompileError::*;
		
		match *self
		{
			Compile(ref string) => write!(f, "compile error '{}'", string),
		}
	}
}

impl Error for ByteCodeCompileError
{
	#[inline(always)]
	fn cause(&self) -> Option<&Error>
	{
		use self::ByteCodeCompileError::*;
		
		match *self
		{
			Compile(_) => None,
		}
	}
}
