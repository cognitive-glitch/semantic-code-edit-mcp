use super::{LanguageCommon, LanguageName, traits::DefaultEditor};
use anyhow::Result;

pub fn language() -> Result<LanguageCommon> {
    let language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
    let editor = Box::new(DefaultEditor::new());

    Ok(LanguageCommon {
        name: LanguageName::Typescript,
        file_extensions: &["ts"],
        language,
        editor,
        validation_query: None,
    })
}
