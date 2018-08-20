// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


bitflags!
{
	/// Flags used for the `bpf_cmd::MAP_UPDATE_ELEM` bpf syscall command.
	pub struct MapUpdateElementFlags: u64
	{
		/// Create a new element or update an existing one.
		///
		/// Known in Linux sources as `BPF_ANY`.
		const Any = 0;
		
		/// Create a new element if it did not exist.
		///
		/// Known in Linux sources as `BPF_NOEXIST`.
		const NoExist = 1;
		
		/// Update an existing element.
		///
		/// Known in Linux sources as `BPF_EXIST`.
		const Exist = 2;
	}
}
