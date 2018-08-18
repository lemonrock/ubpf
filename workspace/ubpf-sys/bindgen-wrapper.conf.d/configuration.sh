# This file is part of ubpf. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT. No part of ubpf, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2018 The developers of ubpf. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ubpf/master/COPYRIGHT.


bindingsName='ubpf'
rootIncludeFileName='ubpf.h'
macosXHomebrewPackageNames=''
alpineLinuxPackageNames='rsync make gcc linux-headers libunwind-dev linux-grsec-dev'
clangAdditionalArguments=''
headersFolderPath='.'
link="ubpf"
link_kind='static-nobundle'

preprocess_before_headersFolderPath()
{
	mkdir -m 0700 -p "$temporaryIncludeFolder"
	cp "$rootOutputFolderPath"/vm/inc/ubpf.h "$temporaryIncludeFolder"/ubpf.h
}

final_chance_to_tweak()
{
	:
}
