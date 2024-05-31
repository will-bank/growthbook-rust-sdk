fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -Dwarnings -Dclippy::unwrap_used

test:
	cargo nextest run