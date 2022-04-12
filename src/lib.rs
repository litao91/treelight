//! [![Version](https://img.shields.io/crates/v/treelight)](https://crates.io/crates/treelight)
//! [![Downloads](https://img.shields.io/crates/d/treelight)](https://crates.io/crates/treelight)
//! [![License](https://img.shields.io/crates/l/treelight)](https://crates.io/crates/treelight)
//! [![Docs](https://docs.rs/treelight/badge.svg)](https://docs.rs/treelight)
//! [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/treelight.svg)](https://web.crev.dev/rust-reviews/crate/treelight/)
//!
//! A syntax highlighter for the web using [tree-sitter](https://github.com/tree-sitter/tree-sitter).
//!
//! ```rust
//! use treelight::*;
//!
//! let code = r#"
//! use thiserror::Error;
//!
//! #[derive(Error, Debug)]
//! pub enum ResponseError {
//!     #[error("api error {0}")]
//!     ApiError(#[from] PaypalError),
//!     #[error("http error {0}")]
//!     HttpError(#[from] reqwest::Error)
//! }
//! "#;
//!
//! let result = highlight_to_html(Language::Rust, code);
//! println!("{}", result);
//! ```

use std::collections::HashMap;

use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};
use v_htmlescape::escape;

pub mod cpp;
pub mod go;
pub mod haskell;
pub mod java;
pub mod javascript;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod scala;
pub mod typescript;

/// The list of supported languages
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Language {
    Rust,
    Javascript,
    JavascriptJsx,
    Typescript,
    TypescriptTsx,
    Python,
    Cpp,
    Java,
    Php,
    Go,
    Scala,
    Haskell,
    Ruby,
}

/// The recognized highlight names, when parsing the code to HTML, the spans will have this name
/// within the class attribute, with the dots replaced by `-`, for example `punctuation.delimiter`
/// will become `<span class="punctuation-delimiter">code here...</span>`.
pub static HIGHLIGHT_NAMES: &[&str] = &[
    "annotation",
    "attribute",
    "boolean",
    "character",
    "character.special",
    "comment",
    "conditional",
    "constant",
    "constant.builtin",
    "constant.macro",
    "constructor",
    "debug",
    "define",
    "error",
    "exception",
    "field",
    "float",
    "function",
    "function.builtin",
    "function.macro",
    "include",
    "keyword",
    "keyword.function",
    "keyword.operator",
    "keyword.return",
    "label",
    "method",
    "namespace",
    "none",
    "number",
    "operator",
    "parameter",
    "parameter.reference",
    "preproc",
    "property",
    "punctuation.delimiter",
    "punctuation.bracket",
    "punctuation.special",
    "repeat",
    "storageclass",
    "string",
    "string.regex",
    "string.escape",
    "string.special",
    "symbol",
    "tag",
    "tag.attribute",
    "tag.delimiter",
    "text",
    "text.strong",
    "text.emphasis",
    "text.underline",
    "text.strike",
    "text.title",
    "text.literal",
    "text.uri",
    "text.math",
    "text.reference",
    "text.environment",
    "text.environment.name",
    "text.note",
    "text.warning",
    "text.danger",
    "todo",
    "type",
    "type.builtin",
    "type.qualifier",
    "type.definition",
    "variable",
    "variable.builtin",
];

lazy_static::lazy_static! {
    static ref LANGUAGE_CONFIGS: HashMap<Language, HighlightConfiguration> = {
        let mut r = HashMap::new();
        for lang in &[Language::Rust, Language::Javascript, Language::JavascriptJsx, Language::Typescript, Language::TypescriptTsx, Language::Python, Language::Cpp, Language::Java, Language::Php, Language::Go, Language::Scala, Language::Haskell, Language::Ruby,
        ] {
    let mut config = {
        match lang {
            Language::Rust => {
                HighlightConfiguration::new(rust::language(), rust::HIGHLIGHT_QUERY, "", "")
                    .unwrap()
            }
            Language::Haskell => HighlightConfiguration::new(
                haskell::language(),
                haskell::HIGHLIGHT_QUERY,
                "",
                haskell::LOCALS_QUERY,
            )
            .unwrap(),
            Language::Go => {
                HighlightConfiguration::new(go::language(), go::HIGHLIGHT_QUERY, "", "").unwrap()
            }
            Language::Ruby => HighlightConfiguration::new(
                ruby::language(),
                ruby::HIGHLIGHT_QUERY,
                "",
                ruby::LOCALS_QUERY,
            )
            .unwrap(),
            Language::Scala => HighlightConfiguration::new(scala::language(), "", "", "").unwrap(),
            Language::Php => HighlightConfiguration::new(
                php::language(),
                php::HIGHLIGHTS_QUERY,
                php::INJECTIONS_QUERY,
                "",
            )
            .unwrap(),
            Language::Typescript => HighlightConfiguration::new(
                typescript::language_typescript(),
                typescript::HIGHLIGHT_QUERY,
                "",
                typescript::LOCALS_QUERY,
            )
            .unwrap(),
            Language::TypescriptTsx => HighlightConfiguration::new(
                typescript::language_tsx(),
                typescript::HIGHLIGHT_QUERY,
                "",
                typescript::LOCALS_QUERY,
            )
            .unwrap(),
            Language::Javascript => HighlightConfiguration::new(
                javascript::language(),
                javascript::HIGHLIGHT_QUERY,
                javascript::INJECTION_QUERY,
                "",
            )
            .unwrap(),
            Language::JavascriptJsx => HighlightConfiguration::new(
                javascript::language(),
                javascript::JSX_HIGHLIGHT_QUERY,
                javascript::INJECTION_QUERY,
                "",
            )
            .unwrap(),
            Language::Python => {
                HighlightConfiguration::new(python::language(), python::HIGHLIGHT_QUERY, "", "")
                    .unwrap()
            }
            Language::Cpp => {
                HighlightConfiguration::new(cpp::language(), cpp::HIGHLIGHT_QUERY, "", "").unwrap()
            }
            Language::Java => {
                HighlightConfiguration::new(java::language(), java::HIGHLIGHT_QUERY, "", "")
                    .unwrap()
            }
        }
    };

    config.configure(&HIGHLIGHT_NAMES);
    r.insert(lang.clone(), config);
        }
        r

    };
}

pub fn highlight_to_html(lang: Language, code: &str, attr_map: &dyn Fn(&str) -> String) -> String {
    let mut highlighter = Highlighter::new();
    let config = LANGUAGE_CONFIGS.get(&lang).unwrap();

    let mut result = String::new();

    let highlights = highlighter
        .highlight(&config, code.as_bytes(), None, |_| None)
        .unwrap();

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                let code_span = escape(code.get(start..end).unwrap()).to_string();
                result.push_str(&code_span);
            }
            HighlightEvent::HighlightStart(s) => {
                let highlight_name = HIGHLIGHT_NAMES.get(s.0).unwrap();
                let attr = attr_map(highlight_name);

                result.push_str(&format!("<span {}>", attr));
            }
            HighlightEvent::HighlightEnd => {
                result.push_str("</span>");
            }
        }
    }

    result
}
