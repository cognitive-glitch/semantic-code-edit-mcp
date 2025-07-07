use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Java,
        &["java"],
        tree_sitter_java::LANGUAGE.into(),
    )
    .build()
}
