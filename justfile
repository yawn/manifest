clean: clean-integration
    cargo clean

[working-directory: 'tests/fixtures/test_crate']
clean-integration:
    cargo clean

doc:
    cargo doc --features build,docs-features,objects --no-deps --open

test: && test-no-objects test-with-objects
    cargo clippy -- -D warnings

test-no-objects: clean
    cargo test --features build,docs-test

test-with-objects: clean
    cargo test --features build,docs-test,objects
