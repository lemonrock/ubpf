// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Struct used by `bpf_cmd::PROG_ATTACH` and `bpf_cmd::PROG_DETACH` bpf syscall commands.
///
/// Anonymously named in original C sources.
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct ProgramAttachOrDetachAttributes
{
	/// Container object to attach to (file descriptor, FD).
	pub target_fd: ContainerObjectFileDescriptor,
	
	/// eBPF program to attach to (file descriptor, FD).
	pub attach_bpf_fd: BpfFileDescriptor,
	
	/// Attach type.
	pub attach_type: bpf_attach_type,
	
	/// Attach flags.
	pub attach_flags: ProgramAttachOrDetachFlags,
}
