# Run cargo clippy with pedantic warnings
clippy:
    cargo clippy -- -W clippy::pedantic

# Check code formatting
format-check:
    cargo fmt --check

# Fix formatting and clippy warnings
fix:
    cargo fmt
    cargo clippy --fix --allow-dirty --allow-staged

# Run tests
test:
    cargo test

# Run all CI checks (format, clippy, test)
ci: format-check clippy test
