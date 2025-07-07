# Codebase Structure

## Root Level
- `README.md` - Main project documentation
- `CLAUDE.md` - Development guidance for Claude
- `Cargo.toml` - Rust package configuration
- `LICENSE` - Apache-2.0 license

## Source Code (`src/`)

### Core Modules
- `main.rs` - Entry point and CLI interface
- `lib.rs` - Library exports and module declarations
- `error.rs` - Comprehensive error type hierarchy

### Editor System (`src/editor/`)
- `editor.rs` - Main editing engine (290+ lines, target for decomposition)
- `edit.rs` - Individual edit operations with validation
- `edit_iterator.rs` - Handles multiple potential edit locations
- `edit_position.rs` - Edit position data structures

### Language Support (`src/languages/`)
- `mod.rs` - Language registry and common types
- `traits.rs` - LanguageEditor trait and default implementation
- Language-specific modules: `rust.rs`, `json.rs`, `python.rs`, etc.
- Each language implements formatting and validation

### Validation System (`src/validation/`)
- `context_validator.rs` - Language-specific semantic validation
- Two-layer approach: context validation + syntax validation

### MCP Tools (`src/tools/`)
- `stage_operation.rs` - Preview edits with diffs
- `commit_staged.rs` - Apply validated changes
- `retarget_staged.rs` - Adjust targeting
- `open_files.rs` - File operations with diff support
- `set_context.rs` - Working directory management

### Core Operations (`src/`)
- `selector.rs` - Node selection and targeting
- `state.rs` - Session state and LRU caching

## Testing (`tests/`)
- `snapshot_runner.rs` - Snapshot testing framework
- `editor_behavior_capture.rs` - Behavior tests for refactoring
- `utf8_boundary_validation.rs` - UTF-8 safety tests
- `error_hierarchy.rs` - Error handling tests
- `snapshots/` - Snapshot test data

## Configuration
- `queries/` - Tree-sitter query definitions
- `.github/` - GitHub Actions and workflows
- `.pre-commit-config.yaml` - Pre-commit hooks