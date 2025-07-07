# Tech Stack

## Language & Edition
- **Rust 2024 edition** (requires nightly due to let chains feature)
- Uses latest Rust features including let chains

## Core Dependencies
- **tree-sitter**: AST parsing for multiple languages
- **tree-sitter-rust**: Rust language support
- **tree-sitter-json**: JSON language support
- **anyhow**: Error handling
- **thiserror**: Error derive macros
- **serde/serde_json**: Serialization
- **clap**: CLI argument parsing
- **schemars**: JSON schema generation

## Text Processing
- **ropey**: Efficient text editing with proper UTF-8 handling
- **diffy**: Diff generation
- **walkdir**: File system traversal

## Language-Specific Support
- Multiple tree-sitter parsers: rust, json, toml, typescript, javascript, python, go, cpp, c, java, php, c-sharp, ruby
- **rustpython-parser**: Python validation
- **taplo**: TOML formatting
- **jsonformat**: JSON formatting

## Development Dependencies
- **tempfile**: Temporary files for testing
- **fastrand**: Random generation
- **lru**: LRU caching
- **prettify-markdown**: Markdown formatting

## Build Configuration
- Optimized release builds with LTO
- Incremental compilation disabled in dev mode
- Multiple codegen optimizations for performance