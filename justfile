# Run all checks
check: fmt-check lint test audit

# Format code
fmt:
    cargo fmt --all

# Verify formatting
fmt-check:
    cargo fmt --all --check

# Lint with clippy
lint:
    cargo clippy --workspace -- -D warnings

# Run tests
test:
    cargo nextest run --workspace

# Audit dependencies
audit:
    cargo deny check

# Generate coverage report
coverage:
    cargo llvm-cov --workspace --html
    @echo "Report at target/llvm-cov/html/index.html"

# Generate coverage summary
coverage-summary:
    cargo llvm-cov --workspace --fail-under-lines 90

# Run the craftty CLI
run *args:
    cargo run -p craftty -- {{args}}

# Generate and open docs
doc:
    cargo doc --workspace --no-deps --open

# Install/update git hooks
hooks:
    cog install-hook --all --overwrite

# Setup development environment
setup:
    ./scripts/setup.sh
