// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


/// A source of bytes.
pub trait ByteSource<'a>
{
	/// Bytes.
	#[inline(always)]
	fn bytes(self) -> io::Result<Cow<'a, [u8]>>;
}

impl<'a> ByteSource<'a> for &'a [u8]
{
	#[inline(always)]
	fn bytes(self) -> io::Result<Cow<'a, [u8]>>
	{
		Ok(Cow::Borrowed(self))
	}
}

impl<'a> ByteSource<'a> for Vec<u8>
{
	#[inline(always)]
	fn bytes(self) -> io::Result<Cow<'a, [u8]>>
	{
		Ok(Cow::Owned(self))
	}
}

impl<'a, 'b> ByteSource<'a> for &'b Path
{
	#[inline(always)]
	fn bytes(self) -> io::Result<Cow<'a, [u8]>>
	{
		let mut file = File::open(self)?;
		let mut buffer = Vec::new();
		file.read_to_end(&mut buffer)?;
		Ok(Cow::Owned(buffer))
	}
}

impl<'a> ByteSource<'a> for PathBuf
{
	#[inline(always)]
	fn bytes(self) -> io::Result<Cow<'a, [u8]>>
	{
		self.as_path().bytes()
	}
}
