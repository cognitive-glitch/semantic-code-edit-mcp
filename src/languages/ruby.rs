use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Ruby,
        &["rb"],
        tree_sitter_ruby::LANGUAGE.into(),
    )
    .build()
}
