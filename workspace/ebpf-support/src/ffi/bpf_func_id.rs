// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Generated from Linux 4.18.3, `source/include/uapi/linux/bpf.h` from <https://elixir.bootlin.com/linux/v4.18.3/source/include/uapi/linux/bpf.h>.
///
/// See also documentation at <https://github.com/iovisor/bcc/blob/master/docs/reference_guide.md>
///
/// Prefixed with `BPF_FUNC_` in Linux sources.
#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum bpf_func_id
{
	#[doc(hidden)]
	unspec = 0,
	
	#[doc(hidden)]
	map_lookup_elem = 1,
	
	#[doc(hidden)]
	map_update_elem = 2,
	
	#[doc(hidden)]
	map_delete_elem = 3,
	
	/// Supported.
	probe_read = 4,
	
	/// Supported.
	ktime_get_ns = 5,
	
	/// Supported.
	trace_printk = 6,
	
	/// Supported.
	get_prandom_u32 = 7,
	
	/// Supported.
	get_smp_processor_id = 8,
	
	#[doc(hidden)]
	skb_store_bytes = 9,
	
	#[doc(hidden)]
	l3_csum_replace = 10,
	
	#[doc(hidden)]
	l4_csum_replace = 11,
	
	#[doc(hidden)]
	tail_call = 12,
	
	#[doc(hidden)]
	clone_redirect = 13,
	
	/// Supported.
	get_current_pid_tgid = 14,
	
	/// Supported.
	get_current_uid_gid = 15,
	
	/// Supported.
	get_current_comm = 16,
	
	#[doc(hidden)]
	get_cgroup_classid = 17,
	
	#[doc(hidden)]
	skb_vlan_push = 18,
	
	#[doc(hidden)]
	skb_vlan_pop = 19,
	
	#[doc(hidden)]
	skb_get_tunnel_key = 20,
	
	#[doc(hidden)]
	skb_set_tunnel_key = 21,
	
	#[doc(hidden)]
	perf_event_read = 22,
	
	#[doc(hidden)]
	redirect = 23,
	
	#[doc(hidden)]
	get_route_realm = 24,
	
	#[doc(hidden)]
	perf_event_output = 25,
	
	#[doc(hidden)]
	skb_load_bytes = 26,
	
	#[doc(hidden)]
	get_stackid = 27,
	
	#[doc(hidden)]
	csum_diff = 28,
	
	#[doc(hidden)]
	skb_get_tunnel_opt = 29,
	
	#[doc(hidden)]
	skb_set_tunnel_opt = 30,
	
	#[doc(hidden)]
	skb_change_proto = 31,
	
	#[doc(hidden)]
	skb_change_type = 32,
	
	#[doc(hidden)]
	skb_under_cgroup = 33,
	
	#[doc(hidden)]
	get_hash_recalc = 34,
	
	#[doc(hidden)]
	get_current_task = 35,
	
	#[doc(hidden)]
	probe_write_user = 36,
	
	#[doc(hidden)]
	current_task_under_cgroup = 37,
	
	#[doc(hidden)]
	skb_change_tail = 38,
	
	#[doc(hidden)]
	skb_pull_data = 39,
	
	#[doc(hidden)]
	csum_update = 40,
	
	#[doc(hidden)]
	set_hash_invalid = 41,
	
	/// Supported.
	get_numa_node_id = 42,
	
	#[doc(hidden)]
	skb_change_head = 43,
	
	#[doc(hidden)]
	xdp_adjust_head = 44,
	
	/// Supported.
	probe_read_str = 45,
	
	#[doc(hidden)]
	get_socket_cookie = 46,
	
	#[doc(hidden)]
	get_socket_uid = 47,
	
	#[doc(hidden)]
	set_hash = 48,
	
	#[doc(hidden)]
	setsockopt = 49,
	
	#[doc(hidden)]
	skb_adjust_room = 50,
	
	#[doc(hidden)]
	redirect_map = 51,
	
	#[doc(hidden)]
	sk_redirect_map = 52,
	
	#[doc(hidden)]
	sock_map_update = 53,
	
	#[doc(hidden)]
	xdp_adjust_meta = 54,
	
	#[doc(hidden)]
	perf_event_read_value = 55,
	
	#[doc(hidden)]
	perf_prog_read_value = 56,
	
	#[doc(hidden)]
	getsockopt = 57,
	
	#[doc(hidden)]
	override_return = 58,
	
	#[doc(hidden)]
	sock_ops_cb_flags_set = 59,
	
	#[doc(hidden)]
	msg_redirect_map = 60,
	
	#[doc(hidden)]
	msg_apply_bytes = 61,
	
	#[doc(hidden)]
	msg_cork_bytes = 62,
	
	#[doc(hidden)]
	msg_pull_data = 63,
	
	#[doc(hidden)]
	bind = 64,
	
	#[doc(hidden)]
	xdp_adjust_tail = 65,
	
	#[doc(hidden)]
	skb_get_xfrm_state = 66,
	
	#[doc(hidden)]
	get_stack = 67,
	
	#[doc(hidden)]
	skb_load_bytes_relative = 68,
	
	#[doc(hidden)]
	fib_lookup = 69,
	
	#[doc(hidden)]
	sock_hash_update = 70,
	
	#[doc(hidden)]
	msg_redirect_hash = 71,
	
	#[doc(hidden)]
	sk_redirect_hash = 72,
	
	#[doc(hidden)]
	lwt_push_encap = 73,
	
	#[doc(hidden)]
	lwt_seg6_store_bytes = 74,
	
	#[doc(hidden)]
	lwt_seg6_adjust_srh = 75,
	
	#[doc(hidden)]
	lwt_seg6_action = 76,
	
	#[doc(hidden)]
	rc_repeat = 77,
	
	#[doc(hidden)]
	rc_keydown = 78,
	
	#[doc(hidden)]
	skb_cgroup_id = 79,
	
	#[doc(hidden)]
	get_current_cgroup_id = 80,
}
