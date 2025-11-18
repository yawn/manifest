bench: bench-inline bench-no-inline

bench-inline: clean-integration
    @echo "Running benchmarks WITH inline(always) (default)..."
    cargo bench --features objects --bench lookup -- --save-baseline inline

bench-no-inline: clean-integration
    @echo "Running benchmarks WITHOUT inline (comparing against inline baseline)..."
    RUSTFLAGS="--cfg no_inline_lookup" cargo bench --features objects --bench lookup -- --baseline inline

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
