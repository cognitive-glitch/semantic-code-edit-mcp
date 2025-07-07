use crate::languages::{LanguageBuilder, LanguageCommon, LanguageName};
use anyhow::Result;

pub fn language() -> Result<LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Typescript,
        &["ts"],
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
    )
    .with_validation_query(include_str!("../../queries/typescript/validation.scm"))
    .build()
}
