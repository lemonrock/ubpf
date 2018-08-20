// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// This helper is a "`printk()`-like" facility for debugging.
///
/// It prints a message defined by format `fmt` (of size `fmt_size`) to file `/sys/kernel/debug/tracing/trace` from DebugFS, if available.
///
/// It can take up to three additional **u64** arguments (as an eBPF helpers, the total number of arguments is limited to five); these can be integer values, pointers, or pointers to nul-terminated C strings.
///
/// Each time the helper is called, it appends a line to the trace.
/// The format of the trace is customizable, and the exact output one will get depends on the options set in `/sys/kernel/debug/tracing/trace_options` (see also the *README* file under the same directory).
/// However, it usually defaults to something like:-
///
/// ``
/// telnet-470   [001] .N.. 419421.045894: 0x00000001: <formatted msg>
/// ``
///
/// In the above:-
///
/// * `telnet` is the name of the current task.
/// * `470` is the PID of the current task.
/// * `001` is the CPU number on which the task is running.
/// * In `.N..`, each character refers to a set of options (whether interrupt requests (IRQs) are enabled, scheduling options, whether hard/softirqs are running, level of `preempt_disabled` respectively).
///   * `N` means that `TIF_NEED_RESCHED` and `PREEMPT_NEED_RESCHED` are set.
/// * `419421.045894` is a timestamp.
/// * `0x00000001` is a fake value used by BPF for the instruction pointer register.
/// * `<formatted msg>` is the message formatted with `fmt`.
///
/// The conversion specifiers supported by `fmt` are similar, but more limited than for printk().
///
/// They are `%d`, `%i`, `%u`, `%x`, `%ld`, `%li`, `%lu`, `%lx`, `%lld`, `%lli`, `%llu`, `%llx`, `%p` and `%s`.
///
/// No modifier (size of field, padding with zeroes, etc.) is available, and the helper will return `-EINVAL` (but print nothing) if it encounters an unknown specifier.
///
/// Also, note that `bpf_trace_printk()` is slow, and should only be used for debugging purposes.
/// For this reason, a notice bloc (spanning several lines) is printed to kernel logs and states that the helper should not be used "for production use" the first time this helper is used (or more precisely, when `trace_printk()` buffers are allocated).
/// For passing values to user space, perf events should be preferred.
///
/// Returns the number of bytes written to the buffer, or a negative error in case of failure (eg `-EINVAL`).
///
/// `int bpf_trace_printk(const char *fmt, u32 fmt_size, ...)`
#[macro_export]
macro_rules! bpf_trace_printk
{
	($format: expr, $fmt_size: expr) =>
	{
		{
			use $crate::libc::*;
			
			const function_identifier: bpf_func_id = $crate::bpf_func_id::trace_printk;
			
			let function_pointer: extern "C" fn(*const c_char, fmt_size: u32, ...) -> c_int = unsafe { ::std::mem::transmute(function_identifier) };
			function_pointer($format, $fmt_size)
		}
	};

	($format: expr, $fmt_size: expr, $value0: expr) =>
	{
		{
			use $crate::libc::*;
			
			const function_identifier: bpf_func_id = $crate::bpf_func_id::trace_printk;
			
			let function_pointer: extern "C" fn(*const c_char, fmt_size: u32, ...) -> c_int = unsafe { ::std::mem::transmute(function_identifier) };
			function_pointer($format, $fmt_size, $value0)
		}
	};

	($format: expr, $fmt_size: expr, $value0: expr, $value1: expr) =>
	{
		{
			use $crate::libc::*;
			
			const function_identifier: bpf_func_id = $crate::bpf_func_id::trace_printk;
			
			let function_pointer: extern "C" fn(*const c_char, fmt_size: u32, ...) -> c_int = unsafe { ::std::mem::transmute(function_identifier) };
			function_pointer($format, $fmt_size, $value0, $value1)
		}
	};

	($format: expr, $fmt_size: expr, $value0: expr, $value1: expr, $value2: expr) =>
	{
		{
			use $crate::libc::*;
			
			const function_identifier: bpf_func_id = $crate::bpf_func_id::trace_printk;
			
			let function_pointer: extern "C" fn(*const c_char, fmt_size: u32, ...) -> c_int = unsafe { ::std::mem::transmute(function_identifier) };
			function_pointer($format, $fmt_size, $value0, $value1, $value2)
		}
	}
}
