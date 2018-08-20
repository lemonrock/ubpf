// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Data for the `bpf_cmd::MAP_LOOKUP_ELEM`, `bpf_cmd::MAP_UPDATE_ELEM` and `bpf_cmd::MAP_DELETE_ELEM` bpf syscall commands.
///
/// Anonymously named in original C sources.
#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct MapElementAttributes
{
	/// File descriptor (FD) of map.
	pub map_fd: MapFileDescriptor,
	
	/// Non-null pointer to key to look up.
	pub key: *mut u8,
	
	/// Either value or next key.
	///
	/// Anonymously named field and struct in original C sources.
	pub value_or_next_key: MapElementValueOrNextKey,
	
	/// Flags.
	pub flags: MapUpdateElementFlags,
}
