# Semantic Code Edit MCP - Project Overview

## Purpose
A Model Context Protocol (MCP) server for semantic code editing using tree-sitter. Provides safe, AST-aware code editing operations that preserve syntax structure and prevent file corruption through comprehensive validation.

## Core Features
- **Multi-language support**: Rust (full), JSON (full), more languages easily added
- **Two-layer validation**: Context validation + syntax validation prevents file corruption
- **Semantic node targeting**: Find nodes by name, type, tree-sitter query, or position
- **Preview mode**: Test operations safely with `preview_only: true`
- **Specialized insertion tools**: Smart, safe insertion at structural boundaries (Rust)
- **Enhanced error messages**: Intelligent suggestions and fuzzy matching
- **Transaction safety**: All edits validated before being applied to files
- **Extensible architecture**: Easy to add support for new programming languages

## Key Architecture
- MCP server that communicates via JSON-RPC over stdin/stdout
- 16 total tools including core editing, analysis, and Rust-specific insertion tools
- Multi-language semantic editing with pluggable language support
- Uses tree-sitter for AST parsing across multiple languages

## Current Status
- Rust edition 2024 (requires nightly)
- 16 MCP tools implemented
- Comprehensive validation system
- Zero file corruption incidents since validation implementation