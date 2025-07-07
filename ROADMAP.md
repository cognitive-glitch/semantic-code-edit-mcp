# Semantic Code Edit MCP - Development Roadmap

## Current Status (Completed)
- ✅ All tests passing
- ✅ Code formatted and clippy-clean
- ✅ Comprehensive integration tests added
- ✅ Documentation added to all modules
- ✅ Error handling improved with anchor validation
- ✅ All initial refactoring tasks completed

## Phase 1: Code Quality Improvements (Next)
1. **Error Handling Refinement**
   - Replace remaining `unwrap()` calls in production code
   - Add integration tests for error cases (as noted in tests/error_hierarchy.rs)
   - Consider using custom error types instead of anyhow in library code

2. **Performance Optimizations**
   - Profile the code for performance bottlenecks
   - Consider lazy-loading language parsers
   - Optimize file caching strategy

3. **Testing Enhancements**
   - Add property-based tests using proptest
   - Increase test coverage for edge cases
   - Add benchmarks for critical paths

## Phase 2: Architectural Decomposition
Based on TDD tests in `tests/editor_decomposition_tdd.rs`:

1. **Extract Validator Module**
   - Move validation logic to `src/editor/validator.rs`
   - Create trait for language-specific validation
   - Support both syntax and semantic validation

2. **Extract Formatter Module**
   - Move formatting logic to `src/editor/formatter.rs`
   - Unified interface for all language formatters
   - Better error handling for formatter failures

3. **Extract DiffGenerator Module**
   - Move diff generation to `src/editor/diff_generator.rs`
   - Support different diff formats
   - Add efficiency metrics calculation

4. **Extract OperationExecutor Module**
   - Move operation execution to `src/editor/operation_executor.rs`
   - Clear separation of concerns
   - Better operation validation

5. **Create Core Orchestrator**
   - Slim down Editor to just orchestration
   - Clear module boundaries
   - Improved testability

## Phase 3: Feature Enhancements
1. **Multi-file Operations**
   - Support for batch operations across files
   - Dependency-aware edits
   - Transaction support

2. **Advanced Selectors**
   - XPath-like selectors for AST nodes
   - Regular expression support in anchors
   - Context-aware selections

3. **Language Support**
   - Add more languages (Go, C++, etc.)
   - Improve existing language support
   - Custom language configuration

4. **Tool Improvements**
   - Better preview formatting
   - Interactive mode for staged operations
   - Undo/redo support

## Phase 4: Integration & Ecosystem
1. **MCP Protocol Enhancements**
   - Support for custom tool parameters
   - Better error reporting through MCP
   - Progress reporting for long operations

2. **CLI Improvements**
   - Standalone CLI tool
   - Configuration file support
   - Plugin system

3. **Documentation & Examples**
   - Comprehensive user guide
   - API documentation
   - Example integrations

## Technical Debt to Address
- Remove TODO comments (found in selector.rs, tools/stage_operation.rs, tests/error_hierarchy.rs)
- Consider replacing anyhow with custom error types for library code
- Investigate the single unwrap() in src/filesystem.rs

## Notes
- Each phase should maintain backward compatibility
- All changes should be driven by tests (TDD)
- Performance should be measured before and after major changes