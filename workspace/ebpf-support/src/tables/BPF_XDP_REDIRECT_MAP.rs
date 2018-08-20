// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Equivalent to `BPF_XDP_REDIRECT_MAP`.
///
/// This table needs to be declared global and static with:-
///
/// ```
/// #[no_mangle]
/// #[link_section = "maps/TABLE_TYPE"]
/// pub static NAME_table_t: BPF_XDP_REDIRECT_MAP = BPF_XDP_REDIRECT_MAP<LEAF>
/// {
/// 	key: 0,
/// 	leaf: 0,
/// 	perf_submit: None,
/// 	perf_submit_skb: None,
/// 	max_entries: MAX_ENTRIES,
/// }
/// ```
///
/// Where `NAME` is some name, `TABLE_TYPE` is either `devmap` or `cpumap` and `MAX_ENTRIES` is the maximum number of entries (which can be zero (0)).
#[allow(missing_docs)]
#[repr(C)]
pub struct BPF_XDP_REDIRECT_MAP<LEAF: Copy>
{
	pub key: u32,
	pub leaf: LEAF,
	pub redirect_map: Option<extern "C" fn(i32, i32) -> u64>,
	pub max_entries: u32,
}
