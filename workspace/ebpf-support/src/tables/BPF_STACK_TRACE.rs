// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Creates a BPF_STACK_TRACE global static entry.
#[macro_export]
macro_rules! BPF_STACK_TRACE
{
	($NAME_table_t: ident, $max_entries: expr) =>
	{
		#[no_mangle]
		#[link_section = "maps/stacktrace"]
		pub static mut $NAME_table_t: BPF_TABLE<i32, bpf_stacktrace> = BPF_TABLE<i32, bpf_stacktrace>
		{
			key: 0,
			leaf: bpf_stacktrace
			{
				ip: [0; bpf_stacktrace::BPF_MAX_STACK_DEPTH],
			},
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
