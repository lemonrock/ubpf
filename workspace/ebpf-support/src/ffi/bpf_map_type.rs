// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// A map type.
///
/// Prefixed with `BPF_MAP_TYPE_` in Linux sources.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum bpf_map_type
{
	UNSPEC = 0,
	HASH = 1,
	ARRAY = 2,
	PROG_ARRAY = 3,
	PERF_EVENT_ARRAY = 4,
	PERCPU_HASH = 5,
	PERCPU_ARRAY = 6,
	STACK_TRACE = 7,
	CGROUP_ARRAY = 8,
	
	/// See also `MapCreateFlags::NoCommonLru`.
	LRU_HASH = 9,
	
	/// See also `MapCreateFlags::NoCommonLru`.
	LRU_PERCPU_HASH = 10,
	
	/// See the struct `bpf_lpm_trie_key`.
	LPM_TRIE = 11,
	
	ARRAY_OF_MAPS = 12,
	HASH_OF_MAPS = 13,
	DEVMAP = 14,
	SOCKMAP = 15,
	CPUMAP = 16,
	XSKMAP = 17,
	SOCKHASH = 18,
}

impl Default for bpf_map_type
{
	#[inline(always)]
	fn default() -> Self
	{
		bpf_map_type::UNSPEC
	}
}
