use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Php,
        &["php"],
        tree_sitter_php::LANGUAGE_PHP.into(),
    )
    .build()
}
