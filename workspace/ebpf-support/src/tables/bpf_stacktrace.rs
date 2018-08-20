// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// A BPF stack trace.
#[repr(C)]
#[allow(missing_docs)]
pub struct bpf_stacktrace
{
	/// ? Instruction Pointer ?
	ip: [u64; bpf_stacktrace::BPF_MAX_STACK_DEPTH],
}

impl bpf_stacktrace
{
	/// Maximum stack depth.
	pub const BPF_MAX_STACK_DEPTH: usize = 127;
}
