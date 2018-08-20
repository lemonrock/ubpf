// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


#[doc(hidden)]
#[macro_export]
macro_rules! bpf_library_license
{
	($license_text: expr $(, $license_array: expr)*) =>
	{
		/// License is $license_text.
		#[no_mangle]
		#[link_section = "license"]
		pub static _license: [u8; 4] = [$($license_array, )*0];
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! bpf_library_license_match
{
	("AGPL-3.0") =>
	{
		bpf_library_license!(AGPL-3.0, 65, 71, 80, 76, 45, 51, 46, 48);
	};
	
	("GPL") =>
	{
		bpf_library_license!(GPL, 71, 80, 76);
	};
	
	("MIT") =>
	{
		bpf_library_license!(MIT, 77, 73, 84);
	};
}

/// Inserts essential sections into resulting shared library ELF.
///
/// Currently only supports the license text for `AGPL-3.0`, `GPL` and `MIT`.
#[macro_export]
macro_rules! bpf_library
{
	($license_text: tt) =>
	{
		bpf_library_license_match!($license_text);
		
		/// Version.
		#[no_mangle]
		#[link_section = "version"]
		pub static _version: u32 = 0xFFFFFFFE;
	}
}
