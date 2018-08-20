// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Creates a BPF_CPUMAP global static entry.
#[macro_export]
macro_rules! BPF_CPUMAP
{
	($NAME_table_t: ident, $max_entries: expr) =>
	{
		#[no_mangle]
		#[link_section = "maps/cpumap"]
		pub static mut $NAME_table_t: BPF_XDP_REDIRECT_MAP<u32> = BPF_XDP_REDIRECT_MAP
		{
			key: 0,
			leaf: 0,
			redirect_map: None,
			max_entries: $max_entries,
		}
	}
}
