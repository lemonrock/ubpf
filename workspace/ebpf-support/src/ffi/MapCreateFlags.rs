// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


bitflags!
{
	/// Flags used for the `bpf_cmd::MAP_UPDATE_ELEM` bpf syscall command.
	pub struct MapCreateFlags: u32
	{
		/// ?
		///
		/// Known in Linux sources as `BPF_F_NO_PREALLOC`.
		const NoPreallocation = 1;
		
		/// Instead of having one common LRU list in the `bpf_map_type::LRU_HASH` and `bpf_map_type::LRU_PERCPU_HASH` map types, use a per-CPU least recently used (LRU) list which can scale and perform better.
		///
		/// Note that the LRU nodes (including free nodes) cannot be moved across different LRU lists.
		///
		/// Known in Linux sources as `BPF_F_NO_COMMON_LRU`.
		const NoCommonLru = 2;
		
		/// A NUMA node is specified in `bpf_attr.map_create.numa_node`
		///
		/// Known in Linux sources as `BPF_F_NUMA_NODE`.
		const NumaNode = 4;
	}
}
