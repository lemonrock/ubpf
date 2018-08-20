// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Data for the `bpf_cmd::TASK_FD_QUERY` bpf syscall command.
///
/// Anonymously named struct in original C sources.
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct TaskFileDescriptorQueryAttributes
{
	/// Program identifier, PID.
	///
	/// Populated on input.
	///
	/// Irrelevant on output.
	pub pid: u32,
	
	/// File descriptor, FD.
	///
	/// Populated on input.
	///
	/// Irrelevant on output.
	pub fd: RawFd,
	
	/// Populated on input.
	///
	/// Irrelevant on output.
	pub flags: TaskFileDescriptorQueryFlags,
	
	/// Size of buffer pointed to by `self.buf`.
	///
	/// Used for both input and output.
	pub buf_len: BufferLength,
	
	/// Used for both input and output.
	///
	/// Pointer to buffer of data with size `self.buf_len`.
	///
	/// Depending on other settings, this `buf` holds:-
	///
	/// * `tp_name` for tracepoint.
	/// * `symbol` for kprobe.
	/// * `filename` for uprobe ?in which case it is a nul-terminated C string?
	pub buf: *mut c_void,
	
	/// Ignored on input.
	///
	/// Populated on output.
	pub prog_id: ProgramIdentifier,
	
	/// Ignored on input.
	///
	/// Populated on output.
	pub fd_type: bpf_task_fd_type,
	
	/// Ignored on input.
	///
	/// Populated on output.
	pub probe_offset: u64,
	
	/// Ignored on input.
	///
	/// Populated on output.
	pub probe_addr: *mut (),
}
