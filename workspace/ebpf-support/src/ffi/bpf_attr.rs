// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Used for the `bpf` syscall.
#[repr(C, align(8))]
pub union bpf_attr
{
	/// Data for `bpf_cmd::MAP_CREATE` bpf syscall command.
	///
	/// Anonymously named field and struct in original C sources.
	pub map_create: MapCreateAttributes,
	
	/// Data for the `bpf_cmd::MAP_LOOKUP_ELEM`, `bpf_cmd::MAP_UPDATE_ELEM` and `bpf_cmd::MAP_DELETE_ELEM` bpf syscall commands.
	///
	/// Anonymously named field and struct in original C sources.
	pub map_element: MapElementAttributes,
	
	/// Data for the `bpf_cmd::PROG_LOAD` bpf syscall command.
	///
	/// Anonymously named field and struct in original C sources.
	pub program_load: ProgramLoadAttributes,
	
	/// Data for the `bpf_cmd::OBJ_PIN` and `bpf_cmd::OBJ_GET` bpf syscall commands.
	///
	/// Anonymously named field and struct in original C sources.
	pub object: ObjectAttributes,
	
	/// Data for the `bpf_cmd::PROG_ATTACH` and `bpf_cmd::PROG_DETACH` bpf syscall commands.
	///
	/// Anonymously named field and struct in original C sources.
	pub program_attach_or_detach: ProgramAttachOrDetachAttributes,
	
	/// Data for the `bpf_cmd::PROG_TEST_RUN` bpf syscall command.
	///
	/// Anonymously named struct in original C sources.
	pub test: ProgramTestRunAttributes,
	
	/// Data for the `bpf_cmd::PROG_GET_NEXT_ID`, `bpf_cmd::MAP_GET_NEXT_ID`, `bpf_cmd::PROG_GET_FD_BY_ID`, `bpf_cmd::MAP_GET_FD_BY_ID` and `bpf::BTF_GET_FD_BY_ID` bpf syscall commands.
	///
	/// Anonymously named field and struct in original C sources.
	pub get_next_identifier: GetNextIdentifierOrFileDescriptorByIdentifierAttributes,
	
	/// Data for the `bpf_cmd::OBJ_GET_INFO_BY_FD` bpf syscall command.
	///
	/// Anonymously named struct in original C sources.
	pub info: ObjectInformationAttributes,
	
	/// Data for the `bpf_cmd::PROG_QUERY` bpf syscall command.
	///
	/// Anonymously named struct in original C sources.
	pub query: ProgramQueryAttributes,
	
	/// Data for the `bpf_cmd::RAW_TRACEPOINT_OPEN` bpf syscall command.
	///
	/// Anonymously named struct in original C sources.
	pub raw_tracepoint: RawTracePointAttributes,
	
	/// Data for the `bpf_cmd::BTF_LOAD` bpf syscall command.
	///
	/// Anonymously named field and struct in original C sources.
	pub btf_load: BtfLoadAttributes,
	
	/// Data for the `bpf_cmd::TASK_FD_QUERY` bpf syscall command.
	///
	/// Anonymously named struct in original C sources.
	pub task_fd_query: TaskFileDescriptorQueryAttributes,
}
