// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Key layout for a `bpf_map_type::LPM_TRIE`.
#[repr(C)]
#[derive(Debug)]
pub struct bpf_lpm_trie_key
{
	/// Prefix length.
	pub prefixlen: u32,
	
	/// Up to 32 for `AF_INET`, 128 for `AF_INET6`.
	pub data: __IncompleteArrayField<u8>,
}
