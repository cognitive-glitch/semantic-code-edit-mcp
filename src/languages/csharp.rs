use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::CSharp,
        &["cs"],
        tree_sitter_c_sharp::LANGUAGE.into(),
    )
    .build()
}
