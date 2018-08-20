// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Struct used by `bpf_cmd::BPF_MAP` bpf syscall command.
///
/// Anonymously named in original C sources.
#[repr(C, align(8))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapCreateAttributes
{
	/// See `bpf_map_type`.
	pub map_type: bpf_map_type,
	
	/// Size of key in bytes.
	pub key_size: u32,
	
	/// Size of value in bytes.
	pub value_size: u32,
	
	/// Maximum number of entries in a map.
	pub max_entries: u32,
	
	/// Map creation flags.
	pub map_flags: MapCreateFlags,
	
	/// File descriptor (FD) pointing to the inner map.
	pub inner_map_fd: MapFileDescriptor,
	
	/// NUMA node (effective only if the `MapCreateFlags:NumaNode` flag is set in the `map_flags` field).
	///
	/// There are normally only NUMA nodes from 0 to 7 inclusive.
	pub numa_node: u32,
	
	/// Map name.
	pub map_name: [c_char; BPF_OBJ_NAME_LEN],
	
	/// `ifindex` (Interface index, starting at ?1) of netdev to create on.
	pub map_ifindex: InterfaceIndex,
	
	/// File descriptor (FD) pointing to BTF type data.
	pub btf_fd: BtfFileDescriptor,
	
	/// BTF `type_id` of the key.
	pub btf_key_type_id: BtfTypeIdentifier,
	
	/// BTF `type_id` of the value.
	pub btf_value_type_id: BtfTypeIdentifier,
}
