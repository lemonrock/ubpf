// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


#![no_main]


#[macro_use] extern crate ebpf_support;
use ::ebpf_support::pt_regs;
use ::ebpf_support::libc::c_char;


bpf_library!("GPL");


// TODO: kprobe__sys_clone(): This is a short-cut for kernel dynamic tracing via kprobes. If the C function begins with kprobe__, the rest is treated as a kernel function name to instrument, in this case, sys_clone(). (event event="sys_clone").
// TODO: All functions needs to be #[inline(always)] unless they are an entry point.
/// By convention, eBPF programs return the following exit codes:-
///
/// * `0`: Success.
/// * `1`: Packet drop.
#[no_mangle]
#[link_section = "kprobe/SyS_clone"]
pub extern "C" fn kprobe__sys_clone(_ctx: *mut pt_regs) -> i32
{
	// "hello from Rust\0"
	static MESSAGE: [c_char; 17] = [104, 101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 114, 117, 115, 116, 10, 0];
	bpf_trace_printk!((&MESSAGE).as_ptr(), MESSAGE.len() as u32);

	0
}
