// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


bitflags!
{
	/// Flags used for the  `bpf_cmd::PROG_GET_NEXT_ID`, `bpf_cmd::MAP_GET_NEXT_ID`, `bpf_cmd::PROG_GET_FD_BY_ID`, `bpf_cmd::MAP_GET_FD_BY_ID` and `bpf::BTF_GET_FD_BY_ID` bpf syscall commands.
	pub struct OpenFlags: u32
	{
		const None = 0;
	}
}
