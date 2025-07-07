# Code Style & Conventions

## Rust Style Guidelines

### General Conventions
- Follow standard Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use `cargo fmt` for automatic formatting
- Run `cargo clippy` for linting
- Prefer explicit error handling over panics

### Error Handling Patterns
- Use `thiserror` derive macro for custom error types
- Comprehensive `SemanticEditError` enum with descriptive variants
- Prefer `Result<T, E>` return types over panics
- Use `expect()` for truly impossible failures with descriptive messages
- Avoid `unwrap()` in production code - use proper error propagation

### Safety & Performance
- Use UTF-8 safe string operations (`str::get()` vs direct indexing)
- Prefer lock-free data structures where possible
- Use `Arc<Mutex<>>` for shared state with proper error handling
- Memory-safe operations with comprehensive validation

### Testing Conventions
- Test-driven development (TDD) approach
- Comprehensive test coverage including edge cases
- Use descriptive test names that document behavior
- Snapshot testing for complex outputs
- Property-based testing for invariants
- UTF-8 boundary testing for string operations

### Module Organization
- Clear separation of concerns
- Each module has single responsibility
- Use traits for extensibility (`LanguageEditor` trait)
- Pluggable architecture for language support

### Documentation
- Comprehensive README with examples
- CLAUDE.md for development guidance
- Inline documentation for public APIs
- Clear error messages with suggestions

### Architecture Patterns
- Two-layer validation (context + syntax)
- Preview mode for safe testing
- Extensible language registry
- MCP tool pattern for server operations