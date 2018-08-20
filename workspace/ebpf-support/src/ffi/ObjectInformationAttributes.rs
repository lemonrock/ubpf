// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Data for the `bpf_cmd::OBJ_GET_INFO_BY_FD` bpf syscall command.
///
/// Anonymously named struct in original C sources.
#[repr(C, align(8))]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ObjectInformationAttributes
{
	/// eBPF program file descriptor (FD).
	pub bpf_fd: BpfFileDescriptor,
	
	/// Size of buffer pointed to by `self.info`.
	pub info_len: BufferLength,
	
	/// Pointer to buffer of data for information with size `self.info_len`.
	pub info: *mut u8,
}
