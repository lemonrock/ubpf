// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Execute a eBPF function (program).
pub trait Execute
{
	/// Execute the eBPF byte code inside this virtual machine.
	///
	/// Will never return `::std::u64::MAX` as this is used internally as a sentinel by ubpf.
	///
	/// stderr needs to be captured as some errors messages (eg divide by zero) are written there.
	#[inline(always)]
	fn execute(&self, memory: &mut [u8]) -> Result<u64, ()>;
}
