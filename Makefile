

env:
	guix environment --manifest=manifest.scm

#watch: 
	#guix environment --manifest=manifest.scm -- find src | entr zig build
