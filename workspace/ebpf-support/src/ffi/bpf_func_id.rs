// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Generated from Linux 4.18.3, `source/include/uapi/linux/bpf.h` from <https://elixir.bootlin.com/linux/v4.18.3/source/include/uapi/linux/bpf.h>.
#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum bpf_func_id
{
	#[doc(hidden)]
	BPF_FUNC_unspec = 0,
	
	/// Supported.
	BPF_FUNC_map_lookup_elem = 1,
	
	/// Supported.
	BPF_FUNC_map_update_elem = 2,
	
	/// Supported.
	BPF_FUNC_map_delete_elem = 3,
	
	#[doc(hidden)]
	BPF_FUNC_probe_read = 4,
	
	/// Supported.
	BPF_FUNC_ktime_get_ns = 5,
	
	/// Supported.
	BPF_FUNC_trace_printk = 6,
	
	/// Supported.
	BPF_FUNC_get_prandom_u32 = 7,
	
	/// Supported.
	BPF_FUNC_get_smp_processor_id = 8,
	
	#[doc(hidden)]
	BPF_FUNC_skb_store_bytes = 9,
	
	#[doc(hidden)]
	BPF_FUNC_l3_csum_replace = 10,
	
	#[doc(hidden)]
	BPF_FUNC_l4_csum_replace = 11,
	
	#[doc(hidden)]
	BPF_FUNC_tail_call = 12,
	
	#[doc(hidden)]
	BPF_FUNC_clone_redirect = 13,
	
	#[doc(hidden)]
	BPF_FUNC_get_current_pid_tgid = 14,
	
	#[doc(hidden)]
	BPF_FUNC_get_current_uid_gid = 15,
	
	#[doc(hidden)]
	BPF_FUNC_get_current_comm = 16,
	
	#[doc(hidden)]
	BPF_FUNC_get_cgroup_classid = 17,
	
	#[doc(hidden)]
	BPF_FUNC_skb_vlan_push = 18,
	
	#[doc(hidden)]
	BPF_FUNC_skb_vlan_pop = 19,
	
	#[doc(hidden)]
	BPF_FUNC_skb_get_tunnel_key = 20,
	
	#[doc(hidden)]
	BPF_FUNC_skb_set_tunnel_key = 21,
	
	#[doc(hidden)]
	BPF_FUNC_perf_event_read = 22,
	
	#[doc(hidden)]
	BPF_FUNC_redirect = 23,
	
	#[doc(hidden)]
	BPF_FUNC_get_route_realm = 24,
	
	#[doc(hidden)]
	BPF_FUNC_perf_event_output = 25,
	
	#[doc(hidden)]
	BPF_FUNC_skb_load_bytes = 26,
	
	#[doc(hidden)]
	BPF_FUNC_get_stackid = 27,
	
	#[doc(hidden)]
	BPF_FUNC_csum_diff = 28,
	
	#[doc(hidden)]
	BPF_FUNC_skb_get_tunnel_opt = 29,
	
	#[doc(hidden)]
	BPF_FUNC_skb_set_tunnel_opt = 30,
	
	#[doc(hidden)]
	BPF_FUNC_skb_change_proto = 31,
	
	#[doc(hidden)]
	BPF_FUNC_skb_change_type = 32,
	
	#[doc(hidden)]
	BPF_FUNC_skb_under_cgroup = 33,
	
	#[doc(hidden)]
	BPF_FUNC_get_hash_recalc = 34,
	
	#[doc(hidden)]
	BPF_FUNC_get_current_task = 35,
	
	#[doc(hidden)]
	BPF_FUNC_probe_write_user = 36,
	
	#[doc(hidden)]
	BPF_FUNC_current_task_under_cgroup = 37,
	
	#[doc(hidden)]
	BPF_FUNC_skb_change_tail = 38,
	
	#[doc(hidden)]
	BPF_FUNC_skb_pull_data = 39,
	
	#[doc(hidden)]
	BPF_FUNC_csum_update = 40,
	
	#[doc(hidden)]
	BPF_FUNC_set_hash_invalid = 41,
	
	/// Supported.
	BPF_FUNC_get_numa_node_id = 42,
	
	#[doc(hidden)]
	BPF_FUNC_skb_change_head = 43,
	
	#[doc(hidden)]
	BPF_FUNC_xdp_adjust_head = 44,
	
	#[doc(hidden)]
	BPF_FUNC_probe_read_str = 45,
	
	#[doc(hidden)]
	BPF_FUNC_get_socket_cookie = 46,
	
	#[doc(hidden)]
	BPF_FUNC_get_socket_uid = 47,
	
	#[doc(hidden)]
	BPF_FUNC_set_hash = 48,
	
	#[doc(hidden)]
	BPF_FUNC_setsockopt = 49,
	
	#[doc(hidden)]
	BPF_FUNC_skb_adjust_room = 50,
	
	#[doc(hidden)]
	BPF_FUNC_redirect_map = 51,
	
	#[doc(hidden)]
	BPF_FUNC_sk_redirect_map = 52,
	
	#[doc(hidden)]
	BPF_FUNC_sock_map_update = 53,
	
	#[doc(hidden)]
	BPF_FUNC_xdp_adjust_meta = 54,
	
	#[doc(hidden)]
	BPF_FUNC_perf_event_read_value = 55,
	
	#[doc(hidden)]
	BPF_FUNC_perf_prog_read_value = 56,
	
	#[doc(hidden)]
	BPF_FUNC_getsockopt = 57,
	
	#[doc(hidden)]
	BPF_FUNC_override_return = 58,
	
	#[doc(hidden)]
	BPF_FUNC_sock_ops_cb_flags_set = 59,
	
	#[doc(hidden)]
	BPF_FUNC_msg_redirect_map = 60,
	
	#[doc(hidden)]
	BPF_FUNC_msg_apply_bytes = 61,
	
	#[doc(hidden)]
	BPF_FUNC_msg_cork_bytes = 62,
	
	#[doc(hidden)]
	BPF_FUNC_msg_pull_data = 63,
	
	#[doc(hidden)]
	BPF_FUNC_bind = 64,
	
	#[doc(hidden)]
	BPF_FUNC_xdp_adjust_tail = 65,
	
	#[doc(hidden)]
	BPF_FUNC_skb_get_xfrm_state = 66,
	
	#[doc(hidden)]
	BPF_FUNC_get_stack = 67,
	
	#[doc(hidden)]
	BPF_FUNC_skb_load_bytes_relative = 68,
	
	#[doc(hidden)]
	BPF_FUNC_fib_lookup = 69,
	
	#[doc(hidden)]
	BPF_FUNC_sock_hash_update = 70,
	
	#[doc(hidden)]
	BPF_FUNC_msg_redirect_hash = 71,
	
	#[doc(hidden)]
	BPF_FUNC_sk_redirect_hash = 72,
	
	#[doc(hidden)]
	BPF_FUNC_lwt_push_encap = 73,
	
	#[doc(hidden)]
	BPF_FUNC_lwt_seg6_store_bytes = 74,
	
	#[doc(hidden)]
	BPF_FUNC_lwt_seg6_adjust_srh = 75,
	
	#[doc(hidden)]
	BPF_FUNC_lwt_seg6_action = 76,
	
	#[doc(hidden)]
	BPF_FUNC_rc_repeat = 77,
	
	#[doc(hidden)]
	BPF_FUNC_rc_keydown = 78,
	
	#[doc(hidden)]
	BPF_FUNC_skb_cgroup_id = 79,
	
	#[doc(hidden)]
	BPF_FUNC_get_current_cgroup_id = 80,
}
