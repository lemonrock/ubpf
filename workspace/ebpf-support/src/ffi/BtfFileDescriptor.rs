// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// BTF file descriptor (FD).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BtfFileDescriptor(pub RawFd);

impl From<RawFd> for BtfFileDescriptor
{
	#[inline(always)]
	fn from(value: RawFd) -> Self
	{
		BtfFileDescriptor(value)
	}
}

impl Into<RawFd> for BtfFileDescriptor
{
	#[inline(always)]
	fn into(self) -> RawFd
	{
		self.0
	}
}
