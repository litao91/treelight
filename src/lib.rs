use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

/// The listo of supported languages
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Language {
    Rust,
    Javascript,
    Python,
    Cpp,
    Java,
}

/// The recognized highlight names, map them to hex colors in a hashmap to create a theme.
pub static HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "label",
    "constant",
    "function.builtin",
    "function.macro",
    "function",
    "keyword",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "string",
    "string.special",
    "tag",
    "escape",
    "type",
    "type.builtin",
    "constructor",
    "variable",
    "variable.builtin",
    "variable.parameter",
    "comment",
];

/// If prepend class is none, then glowtree will be used.
pub fn highlight_to_html(lang: Language, code: &str) -> String {
    let recognized_names: Vec<String> = HIGHLIGHT_NAMES.iter().cloned().map(String::from).collect();

    let mut highlighter = Highlighter::new();

    let mut config = {
        match lang {
            Language::Rust => HighlightConfiguration::new(
                tree_sitter_rust::language(),
                tree_sitter_rust::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap(),
            Language::Java => HighlightConfiguration::new(
                tree_sitter_java::language(),
                tree_sitter_java::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap(),
            Language::Javascript => HighlightConfiguration::new(
                tree_sitter_javascript::language(),
                tree_sitter_javascript::HIGHLIGHT_QUERY,
                tree_sitter_javascript::INJECTION_QUERY,
                "",
            )
            .unwrap(),
            Language::Python => HighlightConfiguration::new(
                tree_sitter_python::language(),
                tree_sitter_python::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap(),
            Language::Cpp => HighlightConfiguration::new(
                tree_sitter_cpp::language(),
                tree_sitter_cpp::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap(),
        }
    };

    config.configure(&recognized_names);

    let mut result = String::new();

    let highlights = highlighter
        .highlight(&config, code.as_bytes(), None, |_| None)
        .unwrap();

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                result.push_str(code.get(start..end).unwrap());
            }
            HighlightEvent::HighlightStart(s) => {
                let name = HIGHLIGHT_NAMES.get(s.0).unwrap().replace(".", "-");

                result.push_str(&format!("<span class='{}'>", name));
            }
            HighlightEvent::HighlightEnd => {
                result.push_str("</span>");
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let code = r#"
use tree_sitter::Parser;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    /// A paypal api error.
    #[error("api error {0}")]
    ApiError(#[from] PaypalError),
    /// A http error.
    #[error("http error {0}")]
    HttpError(#[from] reqwest::Error)
}
"#;

        let result = highlight_to_html(Language::Rust, code);
        println!("{}", result);
    }
}