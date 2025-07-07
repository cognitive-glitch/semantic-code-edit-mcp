# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based MCP (Model Context Protocol) server that provides semantic code editing capabilities using tree-sitter for AST-aware code transformations. It enables precise, syntax-aware code modifications across multiple programming languages.

## Development Commands

### Build
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Run Tests
```bash
# Run all tests
cargo test

# Run a specific test
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

### Lint and Format
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy
```

### Run the Server
```bash
# Run in development
cargo run

# Run release version
cargo run --release

# Install globally
cargo install --path .
semantic-code-edit-mcp
```

## Architecture Overview

### Core Components

1. **Editor System** (`src/editor/`)
   - `Editor`: Main editing engine that validates and applies AST-aware transformations
   - `Edit`: Individual edit operations with built-in validation
   - `EditIterator`: Handles multiple potential edit locations for ambiguous selections
   - Uses `ropey` for efficient rope data structure operations

2. **Language Registry** (`src/languages/`)
   - Extensible system for adding language support
   - Each language implements `LanguageEditor` trait
   - Language-specific formatters and validators:
     - Rust: Uses `rustfmt` for formatting, custom validation queries
     - JSON: Smart indentation detection, `serde_json` validation
     - Python: `rustpython_parser` for validation
     - TOML: `taplo` formatter and validator

3. **Selector System** (`src/selector.rs`)
   - Provides multiple ways to target code:
     - Text anchors (find by string content)
     - AST node types and names
     - Line/column positions
     - Tree-sitter queries
   - Operations: InsertBefore/After, InsertAfterNode, ReplaceRange, ReplaceExact, ReplaceNode

4. **Validation System** (`src/validation/`)
   - Two-layer validation approach:
     - Context validation: Language-specific semantic rules (e.g., no functions in struct fields)
     - Syntax validation: Tree-sitter parsing validation for all languages
   - Prevents file corruption by validating before applying changes

5. **MCP Tools** (`src/tools/`)
   - `stage_operation`: Preview edits with diffs before applying
   - `retarget_staged`: Adjust targeting without rewriting content
   - `commit_staged`: Apply validated changes
   - `open_files`: Load files with diff support
   - `set_context`: Set working directory for relative paths

### Key Design Patterns

1. **Staged Operations**: All edits go through preview → retarget (optional) → commit flow
2. **Session Management**: Persistent sessions with LRU file caching
3. **Error Recovery**: Comprehensive error messages with fuzzy matching suggestions
4. **Safety First**: All operations validate syntax before file modification

## Testing Strategy

The project uses a comprehensive snapshot testing system (`tests/snapshot_runner.rs`):
- Tests are organized in `tests/snapshots/` with input files, args.json, and expected outputs
- Supports filtered test runs via TEST_FILTER environment variable
- Automatic snapshot updates with UPDATE_SNAPSHOTS=1

## Adding New Languages

To add a new language:
1. Add tree-sitter dependency in Cargo.toml
2. Create language module in `src/languages/`
3. Implement `LanguageEditor` trait (can use `DefaultEditor` for basic support)
4. Register in `LanguageRegistry::new()`
5. Add file extensions mapping

## Important Notes

- Requires nightly Rust (uses let chains feature)
- All file paths in tools can be absolute or relative to session context
- Preview mode (`preview_only: true`) is recommended for testing
- The server communicates via JSON-RPC over stdin/stdout