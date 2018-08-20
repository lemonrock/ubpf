// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Task Type.
///
///
/// Prefixed with `BPF_FD_TYPE_` in Linux sources.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum bpf_task_fd_type
{
	/// Trace point (`tp_name`).
	RAW_TRACEPOINT = 0,
	
	/// Trace point (`tp_name`).
	TRACEPOINT = 1,
	
	/// `symbol + offset` or `addr`.
	
	/// `symbol + offset` or `addr`.
	KRETPROBE = 3,		/*  */
	
	/// `filename + offset`.
	UPROBE = 4,
	
	/// `filename + offset`.
	URETPROBE = 5,
}

impl Default for bpf_task_fd_type
{
	#[inline(always)]
	fn default() -> Self
	{
		bpf_task_fd_type::RAW_TRACEPOINT
	}
}
