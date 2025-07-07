# Development Commands

## Build Commands
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Install globally
cargo install --path .
```

## Testing Commands
```bash
# Run all tests
cargo test

# Run specific test
cargo test <test_name>

# Run tests with output
cargo test -- --nocapture

# Run snapshot tests only
cargo test snapshot

# Update snapshot tests
UPDATE_SNAPSHOTS=1 cargo test

# Run filtered snapshot tests
TEST_FILTER="basic_operations" cargo test
TEST_FILTER="basic_operations::insert_after_node" cargo test

# Update specific snapshot test
UPDATE_SNAPSHOTS=1 TEST_FILTER="basic_operations::insert_after_node" cargo test
```

## Linting & Formatting
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy
```

## Running the Server
```bash
# Run in development
cargo run

# Run release version
cargo run --release

# Run as MCP server
semantic-code-edit-mcp
```

## System Commands
```bash
# Git operations
git status
git add .
git commit -m "message"
git push

# File operations (Linux)
ls -la
find . -name "*.rs"
grep -r "pattern" src/
rg "pattern" src/  # Preferred over grep
```

## Special Testing Features
- Snapshot testing system with UPDATE_SNAPSHOTS=1
- Test filtering with TEST_FILTER environment variable
- Comprehensive test coverage including UTF-8 boundary validation
- Editor behavior capture tests for refactoring safety