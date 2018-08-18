// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of ubpf, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


extern "C"
{
	pub fn ubpf_compile(vm: *mut ubpf_vm, errmsg: *mut *mut c_char) -> ubpf_jit_fn;
	pub fn ubpf_create() -> *mut ubpf_vm;
	pub fn ubpf_destroy(vm: *mut ubpf_vm);
	pub fn ubpf_exec(vm: *const ubpf_vm, mem: *mut c_void, mem_len: usize) -> u64;
	pub fn ubpf_load(vm: *mut ubpf_vm, code: *const c_void, code_len: u32, errmsg: *mut *mut c_char) -> c_int;
	pub fn ubpf_load_elf(vm: *mut ubpf_vm, elf: *const c_void, elf_len: usize, errmsg: *mut *mut c_char) -> c_int;
	pub fn ubpf_register(vm: *mut ubpf_vm, idx: c_uint, name: *const c_char, fn_: *mut c_void) -> c_int;
}
