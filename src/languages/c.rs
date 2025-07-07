use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(LanguageName::C, &["c", "h"], tree_sitter_c::LANGUAGE.into()).build()
}
