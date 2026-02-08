.PHONY:	run book qa

run:
	cargo run -p protibuild

book:
	(cd docs && mdbook serve)

qa:
	cargo fmt
	cargo clippy --all-targets --all-features -- -D warnings
	cargo audit
	cargo deny check
	# cargo nextest run --all-targets
	# cargo tarpaulin --all-targets --fail-under 75
