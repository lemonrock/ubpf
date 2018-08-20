// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


#[macro_use] extern crate bitflags;
pub extern crate libc;


/// Linux FFI definitions.
pub mod ffi;


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


//include!("bpf_map_lookup_elem.rs");
//include!("bpf_map_update_elem.rs");
//include!("bpf_map_delete_elem.rs");
//include!("bpf_ktime_get_ns.rs");
include!("bpf_library.rs");
include!("bpf_trace_printk.rs");
//include!("bpf_get_prandom_u32.rs");
//include!("bpf_get_smp_processor_id.rs");
//include!("bpf_get_numa_node_id.rs");
