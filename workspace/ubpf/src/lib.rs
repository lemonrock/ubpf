// This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


#![allow(non_snake_case)]
#![deny(missing_docs)]
#![feature(core_intrinsics)]


//! # ubpf
//!
//! Mid-level rust bindings around the ubpf (libubf) FFI bindings in ubpf-sys.


#[cfg(any(target_os = "android", target_os = "linux"))] include!("lib.cfg.rs");
