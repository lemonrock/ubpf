// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// bpf syscall command.
///
/// Arguments for the command are passed using the union `bpf_attr`.
///
/// Prefixed with `BPF_` in Linux sources.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum bpf_cmd
{
	/// Set the `bpf_attr.map_create` field.
	MAP_CREATE = 0,
	
	/// Set the `bpf_attr.map_element` field.
	MAP_LOOKUP_ELEM = 1,
	
	/// Set the `bpf_attr.map_element` field.
	MAP_UPDATE_ELEM = 2,
	
	/// Set the `bpf_attr.map_element` field.
	MAP_DELETE_ELEM = 3,
	
	MAP_GET_NEXT_KEY = 4,
	
	/// Set the `bpf_attr.program_load` field.
	PROG_LOAD = 5,
	
	/// Set the `bpf_attr.object` field.
	OBJ_PIN = 6,
	
	/// Set the `bpf_attr.object` field.
	OBJ_GET = 7,
	
	PROG_ATTACH = 8,
	
	PROG_DETACH = 9,
	
	PROG_TEST_RUN = 10,
	
	PROG_GET_NEXT_ID = 11,
	
	MAP_GET_NEXT_ID = 12,
	
	PROG_GET_FD_BY_ID = 13,
	
	MAP_GET_FD_BY_ID = 14,
	
	OBJ_GET_INFO_BY_FD = 15,
	
	PROG_QUERY = 16,
	
	RAW_TRACEPOINT_OPEN = 17,
	
	BTF_LOAD = 18,
	
	BTF_GET_FD_BY_ID = 19,
	
	TASK_FD_QUERY = 20,
}
