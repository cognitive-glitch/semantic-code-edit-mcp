use crate::languages::{LanguageBuilder, LanguageCommon, LanguageName};
use anyhow::Result;

pub fn language() -> Result<LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Javascript,
        &["js", "jsx", "mjs", "cjs"],
        tree_sitter_javascript::LANGUAGE.into(),
    )
    .with_validation_query(include_str!("../../queries/javascript/validation.scm"))
    .build()
}
