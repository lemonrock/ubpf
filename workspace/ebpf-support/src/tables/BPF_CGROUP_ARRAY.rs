// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Equivalent to `BPF_CGROUP_ARRAY`.
///
/// Table for cgroup file descriptors.
///
/// This table needs to be declared global and static with:-
///
/// ```
/// #[no_mangle]
/// #[link_section = "maps/cgroup_array"]
/// pub static NAME_table_t: BPF_CGROUP_ARRAY = BPF_CGROUP_ARRAY
/// {
/// 	key: 0,
/// 	leaf: 0,
/// 	perf_submit: None,
/// 	max_entries: MAX_ENTRIES,
/// }
/// ```
///
/// Where `NAME` is some name and `MAX_ENTRIES` is the maximum number of entries (at least one (1)).
#[repr(C)]
#[allow(missing_docs)]
pub struct BPF_CGROUP_ARRAY
{
	pub key: i32,
	pub leaf: u32,
	pub check_current_task: Option<extern "C" fn(i32) -> i32>,
	pub max_entries: u32,
}

/// Creates a BPF_CGROUP_ARRAY global static entry.
#[macro_export]
macro_rules! BPF_CGROUP_ARRAY
{
	($NAME_table_t: ident, $max_entries: expr) =>
	{
		#[no_mangle]
		#[link_section = "maps/cgroup_array"]
		pub static mut $NAME_table_t: BPF_CGROUP_ARRAY = BPF_CGROUP_ARRAY
		{
			key: 0,
			leaf: 0,
			check_current_task: None,
			max_entries: $max_entries,
		}
	}
}
