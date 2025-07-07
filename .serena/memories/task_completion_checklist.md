# Task Completion Checklist

## Before Committing Changes

### Code Quality
- [ ] Run `cargo fmt` to format code
- [ ] Run `cargo clippy` and fix all warnings
- [ ] Ensure no `unwrap()` calls in production code
- [ ] Use proper error handling with `Result<T, E>`

### Testing
- [ ] Run `cargo test` and ensure all tests pass
- [ ] Add tests for new functionality
- [ ] Update snapshot tests if needed with `UPDATE_SNAPSHOTS=1 cargo test`
- [ ] Test UTF-8 boundary cases for string operations
- [ ] Test error conditions and edge cases

### Documentation
- [ ] Update README.md if adding new features
- [ ] Update CLAUDE.md if changing development workflow
- [ ] Add inline documentation for public APIs
- [ ] Include examples for new tools or features

### Validation
- [ ] Test with `preview_only: true` before applying changes
- [ ] Verify two-layer validation (context + syntax) works
- [ ] Test with multiple languages if applicable
- [ ] Ensure no file corruption possible

### Performance
- [ ] Check for memory leaks or excessive allocations
- [ ] Verify UTF-8 safe string operations
- [ ] Use appropriate data structures (`AHashMap`, etc.)
- [ ] Test with large files if relevant

### Architecture
- [ ] Follow single responsibility principle
- [ ] Maintain separation of concerns
- [ ] Use appropriate error types from `SemanticEditError`
- [ ] Consider extensibility for new languages

## TDD Workflow
1. Write failing tests first (RED phase)
2. Implement minimum code to pass tests (GREEN phase)
3. Refactor while keeping tests passing (REFACTOR phase)
4. Maintain comprehensive test coverage

## Release Preparation
- [ ] Update version in Cargo.toml
- [ ] Update changelog/release notes
- [ ] Test release build: `cargo build --release`
- [ ] Verify installation: `cargo install --path .`