check:
    cargo +nightly fmt
    cargo clippy -- -D warnings

test: check
    RUSTFLAGS="-Z macro-backtrace" cargo +nightly test
    # sanity check that the result doesn't differ from nightly
    cargo test