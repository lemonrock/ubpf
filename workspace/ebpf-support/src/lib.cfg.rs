// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


#[macro_use] extern crate bitflags;
pub extern crate libc;


/// Linux FFI definitions.
pub mod ffi;


/// Linux table definitions.
pub mod tables;


use ::libc::c_char;
use ::libc::c_ulong;
use ::libc::c_void;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::marker::PhantomData;
use ::std::mem::transmute;
use ::std::os::unix::io::RawFd;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;


include!("bpf_get_current_comm.rs");
include!("bpf_get_current_pid_tgid.rs");
include!("bpf_get_current_uid_gid.rs");
include!("bpf_get_numa_node_id.rs");
include!("bpf_get_prandom_u32.rs");
include!("bpf_get_smp_processor_id.rs");
include!("bpf_library.rs");
include!("bpf_ktime_get_ns.rs");
include!("bpf_probe_read.rs");
include!("bpf_probe_read_str.rs");
include!("bpf_trace_printk.rs");


/*

 * void *bpf_map_lookup_elem(struct bpf_map *map, const void *key)
 * 	Description
 * 		Perform a lookup in *map* for an entry associated to *key*.
 * 	Return
 * 		Map value associated to *key*, or **NULL** if no entry was
 * 		found.
 *
 
 
 * int bpf_map_update_elem(struct bpf_map *map, const void *key, const void *value, u64 flags)
 * 	Description
 * 		Add or update the value of the entry associated to *key* in
 * 		*map* with *value*. *flags* is one of:
 *
 * 		**BPF_NOEXIST**
 * 			The entry for *key* must not exist in the map.
 * 		**BPF_EXIST**
 * 			The entry for *key* must already exist in the map.
 * 		**BPF_ANY**
 * 			No condition on the existence of the entry for *key*.
 *
 * 		Flag value **BPF_NOEXIST** cannot be used for maps of types
 * 		**BPF_MAP_TYPE_ARRAY** or **BPF_MAP_TYPE_PERCPU_ARRAY**  (all
 * 		elements always exist), the helper would return an error.
 * 	Return
 * 		0 on success, or a negative error in case of failure.
 *
 
 
 * int bpf_map_delete_elem(struct bpf_map *map, const void *key)
 * 	Description
 * 		Delete entry with *key* from *map*.
 * 	Return
 * 		0 on success, or a negative error in case of failure.
 *
 
 
*/
