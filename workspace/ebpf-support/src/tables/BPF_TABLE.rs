// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Equivalent to `BPF_TABLE`.
///
/// This table needs to be declared global and static with:-
///
/// ```
/// #[no_mangle]
/// #[link_section = "maps/TABLE_TYPE"]
/// pub static NAME_table_t: BPF_TABLE<KEY, LEAF> = BPF_TABLE
/// {
/// 	key: 0,
/// 	leaf: 0,
/// 	lookup: None,
/// 	lookup_or_init: None,
/// 	insert: None,
/// 	delete: None,
/// 	call: None,
/// 	increment: None,
/// 	get_stackid: None,
/// 	max_entries: MAX_ENTRIES,
/// 	flags: 0,
/// }
/// ```
///
/// Where `NAME` is some name, `TABLE_TYPE` is one of a ?lower case value in `bpf_map_type`? and `MAX_ENTRIES` is the maximum number of entries can be zero (0)).
///
/// To make the table also 'public', add `#[link_section = "maps/export"]` in addition to `#[link_section = "maps/TABLE_TYPE"]`.
#[allow(missing_docs)]
#[repr(C)]
pub struct BPF_TABLE<KEY: Copy, LEAF: Copy>
{
	pub key: KEY,
	
	pub leaf: LEAF,
	
	pub lookup: Option<extern "C" fn(*mut KEY) -> *mut LEAF>,
	
	pub lookup_or_init: Option<extern "C" fn(*mut KEY, *mut LEAF) -> *mut LEAF>,
	
	pub update: Option<extern "C" fn(*mut KEY, *mut LEAF) -> i32>,
	
	pub insert: Option<extern "C" fn(*mut KEY, *mut LEAF) -> i32>,
	
	pub delete: Option<extern "C" fn(*mut KEY) -> i32>,
	
	/// Second argument is index.
	pub call: Option<extern "C" fn(*mut c_void, i32)>,
	
	pub increment: Option<extern "C" fn(KEY, ...)>,
	
	pub get_stack_id: Option<extern "C" fn(*mut c_void, u64) -> i32>,
	
	pub max_entries: u32,
	
	pub flags: i32,
}
