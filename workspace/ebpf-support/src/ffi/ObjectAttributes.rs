// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Struct used by the `bpf_cmd::OBJ_PIN` and `bpf_cmd::OBJ_GET` bpf syscall commands.
///
/// Anonymously named in original C sources.
#[repr(C, align(8))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectAttributes
{
	/// Pointer to NUL-terminated C string.
	pub pathname: *mut c_char,
	
	/// File descriptor (FD).
	pub bpf_fd: BpfFileDescriptor,
	
	/// ?
	pub file_flags: ObjectFileFlags,
}
