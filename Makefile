.PHONY: watch
watch:
	@cargo check || :
	@ls **/*.rs | entr cargo check

.PHONY: watch/test
watch/test:
	@cargo test || :
	@ls **/*.rs | entr cargo test
