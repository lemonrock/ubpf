// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// Map file descriptor (FD).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MapFileDescriptor(pub RawFd);

impl From<RawFd> for MapFileDescriptor
{
	#[inline(always)]
	fn from(value: RawFd) -> Self
	{
		MapFileDescriptor(value)
	}
}

impl Into<RawFd> for MapFileDescriptor
{
	#[inline(always)]
	fn into(self) -> RawFd
	{
		self.0
	}
}

impl MapFileDescriptor
{
	/// ?
	///
	/// Linux documentation: "when bpf_ldimm64->src_reg == BPF_PSEUDO_MAP_FD, bpf_ldimm64->imm == fd".
	///
	/// Known in Linux sources as `BPF_PSEUDO_MAP_FD`.
	pub const Pseudo: Self = MapFileDescriptor(1);
}
