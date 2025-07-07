use crate::languages::{
    LanguageBuilder, LanguageCommon, LanguageName, traits::LanguageEditor, utils::LineConverter,
};
use anyhow::Result;

pub fn language() -> Result<LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Python,
        &["py", "pyi"],
        tree_sitter_python::LANGUAGE.into(),
    )
    .with_editor(Box::new(PythonEditor))
    .with_validation_query(include_str!("../../queries/python/validation.scm"))
    .build()
}

pub struct PythonEditor;

impl PythonEditor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PythonEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageEditor for PythonEditor {
    fn collect_errors(&self, _tree: &tree_sitter::Tree, content: &str) -> Vec<usize> {
        if let Some(err) =
            rustpython_parser::parse(content, rustpython_parser::Mode::Module, "anonymous.py").err()
        {
            let converter = LineConverter::new(content);
            let byte_offset = usize::from(err.offset);
            vec![converter.offset_to_line(byte_offset)]
        } else {
            vec![]
        }
    }
}
