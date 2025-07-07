use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(LanguageName::Go, &["go"], tree_sitter_go::LANGUAGE.into()).build()
}
