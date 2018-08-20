// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// BTF file descriptor (FD).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BtfTypeIdentifier(u32);

impl From<u32> for BtfTypeIdentifier
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		BtfTypeIdentifier(value)
	}
}

impl Into<u32> for BtfTypeIdentifier
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0
	}
}
