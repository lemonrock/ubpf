// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// A context passed to functions representing the state of registers.
#[repr(C)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct pt_regs
{
	/// The Linux C ABI says this register is callee-preserved, although it isn't saved on kernel entry unless a syscall needs a complete, fully filled `struct pt_regs`.
	pub r15: c_ulong,
	
	/// The Linux C ABI says this register is callee-preserved, although it isn't saved on kernel entry unless a syscall needs a complete, fully filled `struct pt_regs`.
	pub r14: c_ulong,
	
	/// The Linux C ABI says this register is callee-preserved, although it isn't saved on kernel entry unless a syscall needs a complete, fully filled `struct pt_regs`.
	pub r13: c_ulong,
	
	/// The Linux C ABI says this register is callee-preserved, although it isn't saved on kernel entry unless a syscall needs a complete, fully filled `struct pt_regs`.
	pub r12: c_ulong,
	
	/// The Linux C ABI says this register is callee-preserved, although it isn't saved on kernel entry unless a syscall needs a complete, fully filled `struct pt_regs`.
	pub rbp: c_ulong,
	
	/// The Linux C ABI says this register is callee-preserved, although it isn't saved on kernel entry unless a syscall needs a complete, fully filled `struct pt_regs`.
	pub rbx: c_ulong,
	
	/// The Linx C ABI says this register is callee-clobbered; it is always saved on kernel entry.
	pub r11: c_ulong,
	
	/// The Linx C ABI says this register is callee-clobbered; it is always saved on kernel entry.
	pub r10: c_ulong,
	
	/// The Linx C ABI says this register is callee-clobbered; it is always saved on kernel entry.
	pub r9: c_ulong,
	
	/// The Linx C ABI says this register is callee-clobbered; it is always saved on kernel entry.
	pub r8: c_ulong,
	
	/// The Linx C ABI says this register is callee-clobbered; it is always saved on kernel entry.
	pub rax: c_ulong,
	
	/// The Linx C ABI says this register is callee-clobbered; it is always saved on kernel entry.
	pub rcx: c_ulong,
	
	/// The Linx C ABI says this register is callee-clobbered; it is always saved on kernel entry.
	pub rdx: c_ulong,
	
	/// The Linx C ABI says this register is callee-clobbered; it is always saved on kernel entry.
	pub rsi: c_ulong,
	
	/// The Linx C ABI says this register is callee-clobbered; it is always saved on kernel entry.
	pub rdi: c_ulong,
	
	/// On syscall entry, this is the syscall number.
	/// On CPU exception, this is an error code.
	/// On hardware interrupt, it is the IRQ number.
	pub orig_rax: c_ulong,
	
	/// Return frame for `iretq`.
	pub rip: c_ulong,
	
	#[allow(missing_docs)]
	pub cs: c_ulong,
	
	#[allow(missing_docs)]
	pub eflags: c_ulong,
	
	#[allow(missing_docs)]
	pub rsp: c_ulong,
	
	/// Top of stack page.
	pub ss: c_ulong,
}
