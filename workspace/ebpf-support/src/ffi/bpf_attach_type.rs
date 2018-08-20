// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// A program attach type.
///
/// Prefixed with `BPF_` in Linux sources.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum bpf_attach_type
{
	CGROUP_INET_INGRESS = 0,
	CGROUP_INET_EGRESS = 1,
	CGROUP_INET_SOCK_CREATE = 2,
	CGROUP_SOCK_OPS = 3,
	SK_SKB_STREAM_PARSER = 4,
	SK_SKB_STREAM_VERDICT = 5,
	CGROUP_DEVICE = 6,
	SK_MSG_VERDICT = 7,
	CGROUP_INET4_BIND = 8,
	CGROUP_INET6_BIND = 9,
	CGROUP_INET4_CONNECT = 10,
	CGROUP_INET6_CONNECT = 11,
	CGROUP_INET4_POST_BIND = 12,
	CGROUP_INET6_POST_BIND = 13,
	CGROUP_UDP4_SENDMSG = 14,
	CGROUP_UDP6_SENDMSG = 15,
	LIRC_MODE2 = 16,
}

impl Default for bpf_attach_type
{
	#[inline(always)]
	fn default() -> Self
	{
		bpf_attach_type::CGROUP_INET_INGRESS
	}
}
