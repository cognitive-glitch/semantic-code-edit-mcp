use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Cpp,
        &["cpp", "cxx", "cc", "c++", "hpp", "hxx", "h++"],
        tree_sitter_cpp::LANGUAGE.into(),
    )
    .build()
}
