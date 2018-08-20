// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


#[allow(missing_docs)]
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(PhantomData<T>);

impl<T> __IncompleteArrayField<T>
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn new() -> Self
	{
		__IncompleteArrayField(PhantomData)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn as_ptr(&self) -> *const T
	{
		unsafe{ transmute(self) }
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn as_mut_ptr(&mut self) -> *mut T
	{
		unsafe { transmute(self) }
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn as_slice(&self, length: usize) -> &[T]
	{
		unsafe { from_raw_parts(self.as_ptr(), length) }
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn as_mut_slice(&mut self, length: usize) -> &mut [T]
	{
		unsafe { from_raw_parts_mut(self.as_mut_ptr(), length) }
	}
}

impl<T> Debug for __IncompleteArrayField<T>
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result
	{
		fmt.write_str("__IncompleteArrayField")
	}
}

impl<T> Clone for __IncompleteArrayField<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self::new()
	}
}

impl<T> Copy for __IncompleteArrayField<T>
{
}
