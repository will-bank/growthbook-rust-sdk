fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -Dwarnings -Dclippy::unwrap_used

test:
	@cargo watch -q -c -x 'nextest run ${FILTER} --no-capture'

run:
	cargo watch -q -c -x 'run --bin main'