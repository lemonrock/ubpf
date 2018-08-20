// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Value or next key for the `bpf_cmd::MAP_LOOKUP_ELEM`, `bpf_cmd::MAP_UPDATE_ELEM` and `bpf_cmd::MAP_DELETE_ELEM` commands.
///
/// Anonymously named in original C sources.
#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub union MapElementValueOrNextKey
{
	/// Non-null pointer to a value.
	pub value: *mut u8,
	
	/// Non-null pointer to a next key.
	pub next_key: *mut u8,
}
