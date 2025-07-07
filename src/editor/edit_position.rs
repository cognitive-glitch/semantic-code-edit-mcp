//! Edit position tracking for precise code modifications.
//!
//! This module provides the `EditPosition` struct which tracks byte positions
//! for edit operations within source code. Features include:
//! - Byte-precise position tracking
//! - Support for both insert and replace operations
//! - Serialization support for staging operations
//! - Integration with rope-based text manipulation
//! - UTF-8 safe positioning

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct EditPosition {
    pub start_byte: usize,
    pub end_byte: Option<usize>, // None for insert, Some for replace
}
