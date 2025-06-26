fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -Dwarnings -Dclippy::unwrap_used

test:
	@cargo watch -q -c -x 'nextest run ${FILTER} --no-capture'

test_mocker:
	@cargo test --features mocker --test mocker_test

test_oneshot:
	cargo nextest run
