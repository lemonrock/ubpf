// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Equivalent to `BPF_PERF_ARRAY`.
///
/// Table for reading hardware performance CPU counters.
///
/// This table needs to be declared global and static with:-
///
/// ```
/// #[no_mangle]
/// #[link_section = "maps/perf_array"]
/// pub static NAME_table_t: BPF_PERF_ARRAY = BPF_PERF_ARRAY
/// {
/// 	key: 0,
/// 	leaf: 0,
/// 	perf_submit: None,
/// 	perf_submit_skb: None,
/// 	max_entries: MAX_ENTRIES,
/// }
/// ```
///
/// Where `NAME` is some name and `MAX_ENTRIES` is the maximum number of entries (which can be zero (0)).
#[allow(missing_docs)]
#[repr(C)]
pub struct BPF_PERF_ARRAY
{
	pub key: i32,
	
	pub leaf: u32,
	
	/// First argument is index.
	pub perf_read: Option<extern "C" fn(i32) -> u64>,
	
	pub perf_counter_value: Option<extern "C" fn(i32, *mut c_void, u32) -> i32>,
	
	pub max_entries: u32,
}

/// Creates a BPF_PERF_ARRAY global static entry.
#[macro_export]
macro_rules! BPF_PERF_ARRAY
{
	($NAME_table_t: ident, $max_entries: expr) =>
	{
		#[no_mangle]
		#[link_section = "maps/perf_array"]
		pub static mut $NAME_table_t: BPF_PERF_ARRAY = BPF_PERF_ARRAY
		{
			key: 0,
			leaf: 0,
			perf_read: None,
			perf_counter_value: None,
			max_entries: $max_entries,
		}
	}
}
