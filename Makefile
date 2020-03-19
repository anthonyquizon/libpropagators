.PHONY: watch
watch:
	@cargo check || :
	@ls ./src/**/*.rs | entr cargo check

.PHONY: watch/test
watch/test:
	@cargo test || :
	@ls ./src/**/*.rs ./tests/**/*.rs | entr cargo test -- --nocapture
