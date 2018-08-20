// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Data for the `bpf_cmd::PROG_QUERY` bpf syscall command.
///
/// Anonymously named struct in original C sources.
#[repr(C, align(8))]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ProgramQueryAttributes
{
	/// Container object to attach to (file descriptor, FD).
	pub target_fd: ContainerObjectFileDescriptor,
	
	#[allow(missing_docs)]
	pub attach_type: bpf_attach_type,
	
	#[allow(missing_docs)]
	pub query_flags: ProgramQueryFlags,
	
	#[allow(missing_docs)]
	pub attach_flags: ProgramAttachOrDetachFlags,
	
	/// Pointer to buffer of data for program ids with size `self.prog_cnt`.
	pub prog_ids: ProgramIdentifier,
	
	/// Size of buffer pointed to by `self.prog_ids`.
	pub prog_cnt: BufferLength,
}
