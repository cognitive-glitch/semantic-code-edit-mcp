use super::{LanguageBuilder, LanguageCommon, LanguageName, traits::LanguageEditor};
use anyhow::{Result, anyhow};
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

pub fn language() -> Result<LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Rust,
        &["rs"],
        tree_sitter_rust::LANGUAGE.into(),
    )
    .with_editor(Box::new(RustEditor))
    .with_validation_query(include_str!("../../queries/rust/validation.scm"))
    .build()
}

struct RustEditor;

impl LanguageEditor for RustEditor {
    fn format_code(&self, source: &str) -> Result<String> {
        let mut child = Command::new("rustfmt")
            .args(["--emit", "stdout", "--edition", "2024"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(source.as_bytes())?;
            drop(stdin);
        }

        let mut stdout = String::new();
        if let Some(mut out) = child.stdout.take() {
            out.read_to_string(&mut stdout)?;
        }

        let mut stderr = String::new();
        if let Some(mut err) = child.stderr.take() {
            err.read_to_string(&mut stderr)?;
        }

        if child.wait()?.success() {
            Ok(stdout)
        } else {
            Err(anyhow!(stderr))
        }
    }
}
