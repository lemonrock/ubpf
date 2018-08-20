// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Data for the `bpf_cmd::PROG_TEST_RUN` bpf syscall command.
///
/// Anonymously named struct in original C sources.
#[repr(C, align(8))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramTestRunAttributes
{
	/// Program file descriptor.
	pub prog_fd: BpfFileDescriptor,
	
	/// ?Value returned by test run?
	pub retval: u32,
	
	/// Size of buffer pointed to by `self.data_in`.
	pub data_size_in: BufferLength,
	
	/// Size of buffer pointed to by `self.data_out`.
	pub data_size_out: BufferLength,
	
	/// Pointer to buffer of data in with size `self.data_size_in`.
	pub data_in: *mut u8,
	
	/// Pointer to buffer of data out with size `self.data_size_out`.
	pub data_out: *mut u8,
	
	/// ?
	pub repeat: u32,
	
	/// ?
	pub duration: u32,
}
