// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Data for the `bpf_cmd::BTF_LOAD` bpf syscall command.
///
/// Anonymously named struct in original C sources.
#[repr(C, align(8))]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BtfLoadAttributes
{
	/// Pointer to a buffer of `BTF` data to load with length `self.btf_size`.
	pub btf: *const u8,
	
	/// Pointer to a buffer for storing verifier messages with length `self.btf_log_size`.
	pub btf_log_buf: *mut u8,
	
	/// Size of buffer `self.btf`.
	pub btf_size: BufferLength,
	
	/// Length of verifier message buffer pointed to be `self.btf_log_buf`.
	pub btf_log_size: BufferLength,
	
	/// Verifier verbosity.
	pub btf_log_level: VerifierVerbosityLevel,
}
