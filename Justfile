# Builds the crate
build:
    cargo build

# Runs tests
test:
    cargo test

# Checks linting and formatting
check-lint:
    cargo clippy -- -D warnings
    cargo fmt -- --check

# Creates README.md
readme:
    cargo readme > README.md
    sed -i '' 's/\[\(`[^`]*`\)\]/\1/g' README.md
