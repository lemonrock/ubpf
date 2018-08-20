// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Equivalent to `BPF_PERF_OUTPUT`.
///
/// Table for reading hardware performance CPU counters.
///
/// This table needs to be declared global and static with:-
///
/// ```
/// #[no_mangle]
/// #[link_section = "maps/perf_output"]
/// pub static NAME_table_t: BPF_PERF_OUTPUT = BPF_PERF_OUTPUT
/// {
/// 	key: 0,
/// 	leaf: 0,
/// 	perf_submit: None,
/// 	perf_submit_skb: None,
/// 	max_entries: 0,
/// }
/// ```
///
/// Where `NAME` is some name.
#[allow(missing_docs)]
#[repr(C)]
pub struct BPF_PERF_OUTPUT
{
	pub key: i32,
	pub leaf: u32,
	pub perf_submit: Option<extern "C" fn(*mut c_void, *mut c_void, u32) -> i32>,
	pub perf_submit_skb: Option<extern "C" fn(*mut c_void, u32, *mut c_void, u32) -> i32>,
	pub max_entries: u32,
}

/// Creates a BPF_PERF_OUTPUT global static entry.
#[macro_export]
macro_rules! BPF_PERF_OUTPUT
{
	($NAME_table_t: ident, $max_entries: expr) =>
	{
		#[no_mangle]
		#[link_section = "maps/perf_output"]
		pub static mut $NAME_table_t: BPF_PERF_OUTPUT = BPF_PERF_OUTPUT
		{
			key: 0,
			leaf: 0,
			perf_submit: None,
			perf_submit_skb: None,
			max_entries: $max_entries,
		}
	}
}
