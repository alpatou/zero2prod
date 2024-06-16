watch: 
	cargo watch -x check
	
chain-watch:
	cargo watch -x check -x test -x run 
	
quality-without-tests:
	cargo tarpaulin --ignore-test

linting: 
	cargo clippy -- -D warnings

format: 
	cargo fmt -- --check

audit: 
	cargo audit