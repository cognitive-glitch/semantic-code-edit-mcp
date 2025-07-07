# Changelog

## [Unreleased]

### Added
- Comprehensive integration tests suite (`tests/integration_test.rs`) with 12 test cases
- Error handling integration tests (`tests/error_handling_integration.rs`) with 11 test cases
- Development roadmap (`ROADMAP.md`) outlining future phases
- Changelog to track project progress

### Changed
- Enhanced `Editor::new()` with anchor existence validation
- Added syntax validation for ReplaceNode operations using tree-sitter
- Improved JavaScript validation queries to be less restrictive
- Updated test expectations to match new validation behavior

### Fixed
- Fixed anchor issues in `editor_decomposition_tdd.rs` tests
- Fixed test failure in `editor_behavior_capture.rs` to expect validation errors
- Resolved all clippy warnings:
  - Added `is_empty()` method to `StatsLruCache`
  - Replaced unnecessary match statements
  - Fixed logic bug in condition check

### Documentation
- Added comprehensive module-level documentation to all language modules
- Added documentation to all tool modules
- Documented core editor components and their responsibilities

## Notes
- All TODO comments and unwrap() calls in production code have been reviewed
- The single unwrap() in filesystem.rs is actually `expect()` with proper error message for mutex poisoning, which is appropriate
- Project is now ready for the next phase of development (architectural decomposition)