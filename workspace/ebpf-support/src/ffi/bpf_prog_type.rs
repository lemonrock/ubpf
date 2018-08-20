// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// A map type.
///
/// Prefixed with `BPF_PROG_TYPE_` in Linux sources.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum bpf_prog_type
{
	UNSPEC = 0,
	SOCKET_FILTER = 1,
	KPROBE = 2,
	SCHED_CLS = 3,
	SCHED_ACT = 4,
	TRACEPOINT = 5,
	XDP = 6,
	PERF_EVENT = 7,
	CGROUP_SKB = 8,
	CGROUP_SOCK = 9,
	LWT_IN = 10,
	LWT_OUT = 11,
	LWT_XMIT = 12,
	SOCK_OPS = 13,
	SK_SKB = 14,
	CGROUP_DEVICE = 15,
	SK_MSG = 16,
	RAW_TRACEPOINT = 17,
	CGROUP_SOCK_ADDR = 18,
	LWT_SEG6LOCAL = 19,
	LIRC_MODE2 = 20,
}

impl Default for bpf_prog_type
{
	#[inline(always)]
	fn default() -> Self
	{
		bpf_prog_type::UNSPEC
	}
}
