// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Creates a BPF_PROG_ARRAY global static entry.
#[macro_export]
macro_rules! BPF_PROG_ARRAY
{
	($NAME_table_t: ident, $max_entries: expr) =>
	{
		#[no_mangle]
		#[link_section = "maps/prog"]
		pub static mut $NAME_table_t: BPF_TABLE<u32, u32> = BPF_TABLE
		{
			key: 0,
			leaf: 0,
			lookup: None,
			lookup_or_init: None,
			insert: None,
			delete: None,
			call: None,
			increment: None,
			get_stackid: None,
			max_entries: $max_entries,
			flags: 0,
		}
	}
}
