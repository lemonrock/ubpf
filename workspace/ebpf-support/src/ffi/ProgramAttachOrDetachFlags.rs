// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


bitflags!
{
	/// Flags used for the `bpf_cmd::PROG_ATTACH` and `bpf_cmd::PROG_DETACH` bpf syscall commands.
	pub struct ProgramAttachOrDetachFlags: u32
	{
		/// Known in Linux sources as `BPF_F_ALLOW_OVERRIDE`.
		const AllowOverride = 1;
		
		/// Known in Linux sources as `BPF_F_ALLOW_MULTI`.
		const AllowMultiple = 2;
	}
}
