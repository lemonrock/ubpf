// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Data for the `bpf_cmd::RAW_TRACEPOINT_OPEN` bpf syscall command.
///
/// Anonymously named struct in original C sources.
#[repr(C, align(8))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawTracePointAttributes
{
	/// Pointer to a nul-terminated C string.
	pub name: *const c_char,
	
	/// Program file descriptor.
	pub prog_fd: RawFd,
}
