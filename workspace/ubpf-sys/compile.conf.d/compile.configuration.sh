# This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of ubpf, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2018 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


compile_library_name='ubpf'

compile_library()
{
	compile_ubpf_make()
	{
		local crossCompilerPrefix="$1"
		
		cd "$rootOutputFolderPath" 1>/dev/null 2>/dev/null

			# This is for our own patches to buildtools/check-experimental-syms.sh
			export CROSS="$crossCompilerPrefix"

			CC="$crossCompilerPrefix"cc AR="$crossCompilerPrefix"ar make \
				-j $numberOfMakeJobs \
				-C vm \
				1>&2

		cd - 1>/dev/null 2>/dev/null
	}

	local configurationName='x86_64-native-linuxapp-gcc'
	local crossCompilerPrefix='x86_64-linux-musl-'

	compile_ubpf_make "$crossCompilerPrefix" 2>&1
}

cargo_key_value_pairs()
{
	cargo_key_value_pairs_link_lib 'static-nobundle' 'ubpf'

	# Search path
	cargo_key_value_pairs_search 'native' "$OUT_DIR"/root/usr/lib

	# Not used by us, but potentially used by downstream crates.
	cargo_key_value_pairs_other 'root' "$OUT_DIR"/root
	cargo_key_value_pairs_other 'include' "$OUT_DIR"/root/usr/include/ubpf
	cargo_key_value_pairs_other 'libdir' "$OUT_DIR"/root/usr/lib
}
