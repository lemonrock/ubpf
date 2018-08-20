// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Data for the `bpf_cmd::PROG_LOAD` bpf syscall command.
///
/// Anonymously named in original C sources.
#[repr(C, align(8))]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ProgramLoadAttributes
{
	/// Program type.
	pub prog_type: bpf_prog_type,
	
	/// Length of instruction byte code buffer pointed to be `self.insns`.
	pub insn_cnt: BufferLength,
	
	/// Pointer to a byte code buffer with length `self.insn_cnt`.
	pub insns: *const u8,
	
	/// Pointer to a nul-terminated C string for the license, eg `GPL\0`.
	pub license: *const c_char,
	
	/// Verbosity of verifier.
	pub log_level: VerifierVerbosityLevel,
	
	/// Length of verifier message buffer pointed to be `self.log_buf`.
	pub log_size: BufferLength,
	
	/// Pointer to a buffer for storing verifier messages with length `self.log_size`.
	pub log_buf: *mut u8,
	
	/// Only checked whtn `self.prog_type == bpf_prog_type::KPROBE`.
	pub kern_version: u32,
	
	/// ?
	pub prog_flags: ProgramLoadFlags,
	
	/// Program name.
	pub prog_name: [c_char; BPF_OBJ_NAME_LEN],
	
	/// `ifindex` (Interface index, starting at ?1) of netdev to prepare for.
	///
	/// For some program types (`self.prog_type`) the expected attach type must be known at load time to verify the attach type specific parts of the program (such as context accesses, allowed helpers, etc).
	pub prog_ifindex: InterfaceIndex,
	
	/// ?
	pub expected_attach_type: bpf_attach_type,
}
