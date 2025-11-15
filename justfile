clean: clean-integration
    cargo clean

[working-directory: 'tests/fixtures/test_crate']
clean-integration:
    cargo clean

doc:
    cargo doc --features build --no-deps

test: && test-no-objects test-with-objects
    cargo clippy -- -D warnings

test-no-objects: clean
    cargo test --features build,doctest

test-with-objects: clean
    cargo test --features build,doctest,objects
