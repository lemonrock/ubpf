// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Creates a BPF_LPM_TRIE global static entry.
#[macro_export]
macro_rules! BPF_LPM_TRIE
{
	($NAME_table_t: ident) =>
	{
		#[no_mangle]
		#[link_section = "maps/lpm_trie"]
		pub static mut $NAME_table_t: BPF_TABLE<u64, u64> = BPF_TABLE
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
			max_entries: 10240,
			flags: $crate::ffi::NoPreallocation::NoPreallocation as u32 as u64,
		}
	};
	
	($NAME_table_t: ident, $key_type: ty, $key_default: expr) =>
	{
		#[no_mangle]
		#[link_section = "maps/lpm_trie"]
		pub static mut $NAME_table_t: BPF_TABLE<$key_type, u64> = BPF_TABLE
		{
			key: $key_default,
			leaf: 0,
			lookup: None,
			lookup_or_init: None,
			insert: None,
			delete: None,
			call: None,
			increment: None,
			get_stackid: None,
			max_entries: 10240,
			flags: $crate::ffi::NoPreallocation::NoPreallocation as u32 as u64,
		}
	};
	
	($NAME_table_t: ident, $key_type: ty, $key_default: expr, $leaf_type: ty, $leaf_default: expr) =>
	{
		#[no_mangle]
		#[link_section = "maps/lpm_trie"]
		pub static mut $NAME_table_t: BPF_TABLE<$key_type, $leaf_type> = BPF_TABLE
		{
			key: $key_default,
			leaf: $leaf_default,
			lookup: None,
			lookup_or_init: None,
			insert: None,
			delete: None,
			call: None,
			increment: None,
			get_stackid: None,
			max_entries: 10240,
			flags: $crate::ffi::NoPreallocation::NoPreallocation as u32 as u64,
		}
	};
	
	($NAME_table_t: ident, $key_type: ty, $key_default: expr, $leaf_type: ty, $leaf_default: expr, $max_entries: expr) =>
	{
		#[no_mangle]
		#[link_section = "maps/lpm_trie"]
		pub static mut $NAME_table_t: BPF_TABLE<$key_type, $leaf_type> = BPF_TABLE
		{
			key: $key_default,
			leaf: $leaf_default,
			lookup: None,
			lookup_or_init: None,
			insert: None,
			delete: None,
			call: None,
			increment: None,
			get_stackid: None,
			max_entries: $max_entries,
			flags: $crate::ffi::NoPreallocation::NoPreallocation as u32 as u64,
		}
	}
}
