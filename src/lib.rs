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


use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};
use v_htmlescape::escape;

pub mod languages;
pub mod queries;

use languages::Language;

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
  static ref RUST_CONFIG: HighlightConfiguration =  {
      let mut config = HighlightConfiguration::new(languages::rust(),
                                &queries::get_query(&queries::HIGHLIGHTS, "rust").unwrap(),
                                &queries::get_query(&queries::INJECTIONS, "rust").unwrap(),
                                &queries::get_query(&queries::LOCALS, "rust").unwrap()).unwrap();
      config.configure(&HIGHLIGHT_NAMES);
      config

  };

  static ref GO_CONFIG: HighlightConfiguration =  {
      let mut config = HighlightConfiguration::new(languages::go(),
                                &queries::get_query(&queries::HIGHLIGHTS, "go").unwrap(),
                                &queries::get_query(&queries::INJECTIONS, "go").unwrap(),
                                &queries::get_query(&queries::LOCALS, "go").unwrap())

        .unwrap();
      config.configure(&HIGHLIGHT_NAMES);
      config
  };

  static ref TS_CONFIG: HighlightConfiguration = {
      let mut config = HighlightConfiguration::new(languages::typescript(),
                                &queries::get_query(&queries::HIGHLIGHTS, "typescript").unwrap(),
                                &queries::get_query(&queries::INJECTIONS, "typescript").unwrap(),
                                &queries::get_query(&queries::LOCALS, "typescript").unwrap())

        .unwrap();
      config.configure(&HIGHLIGHT_NAMES);
      config
  };

  static ref TSX_CONFIG: HighlightConfiguration = {
      let mut config = HighlightConfiguration::new(languages::tsx(),
                                &queries::get_query(&queries::HIGHLIGHTS, "tsx").unwrap(),
                                &queries::get_query(&queries::INJECTIONS, "tsx").unwrap(),
                                &queries::get_query(&queries::LOCALS, "tsx").unwrap())

        .unwrap();
      config.configure(&HIGHLIGHT_NAMES);
      config

    };

  static ref JS_CONFIG: HighlightConfiguration = {
      let mut config = HighlightConfiguration::new(languages::javascript(),
                                &queries::get_query(&queries::HIGHLIGHTS, "javascript").unwrap(),
                                &queries::get_query(&queries::INJECTIONS, "javascript").unwrap(),
                                &queries::get_query(&queries::LOCALS, "javascript").unwrap())

        .unwrap();
      config.configure(&HIGHLIGHT_NAMES);
      config

    };

  static ref JSX_CONFIG: HighlightConfiguration = {
      let mut config = HighlightConfiguration::new(languages::javascript(),
                                &queries::get_query(&queries::HIGHLIGHTS, "jsx").unwrap(),
                                &queries::get_query(&queries::INJECTIONS, "javascript").unwrap(),
                                &queries::get_query(&queries::LOCALS, "javascript").unwrap())

        .unwrap();
      config.configure(&HIGHLIGHT_NAMES);
      config

    };

  static ref PYTHON_CONFIG: HighlightConfiguration = {
      let mut config = HighlightConfiguration::new(languages::python(),
                                &queries::get_query(&queries::HIGHLIGHTS, "python").unwrap(),
                                &queries::get_query(&queries::INJECTIONS, "python").unwrap(),
                                &queries::get_query(&queries::LOCALS, "python").unwrap())

        .unwrap();
      config.configure(&HIGHLIGHT_NAMES);
      config

    };

  static ref CPP_CONFIG: HighlightConfiguration = {
      let mut config = HighlightConfiguration::new(languages::cpp(),
                                &queries::get_query(&queries::HIGHLIGHTS, "cpp").unwrap(),
                                &queries::get_query(&queries::INJECTIONS, "cpp").unwrap(),
                                &queries::get_query(&queries::LOCALS, "cpp").unwrap())

        .unwrap();
      config.configure(&HIGHLIGHT_NAMES);
      config

    };
  static ref JAVA_CONFIG: HighlightConfiguration = {
      let mut config = HighlightConfiguration::new(languages::java(),
                                &queries::get_query(&queries::HIGHLIGHTS, "java").unwrap(),
                                &queries::get_query(&queries::INJECTIONS, "java").unwrap(),
                                &queries::get_query(&queries::LOCALS, "java").unwrap())

        .unwrap();
      config.configure(&HIGHLIGHT_NAMES);
      config

    };

}

fn load_language<'a>(language: &Language) -> &'a HighlightConfiguration {
    match language {
        Language::Rust => &*RUST_CONFIG,
        Language::Javascript => &*JS_CONFIG,
        Language::JavascriptJsx => &*JSX_CONFIG,
        Language::Typescript => &*TS_CONFIG,
        Language::TypescriptTsx => &*TSX_CONFIG,
        Language::Python => &*PYTHON_CONFIG,
        Language::Cpp => &*CPP_CONFIG,
        Language::Java => &*JAVA_CONFIG,
        Language::Go => &*GO_CONFIG,
    }
}

pub fn highlight_to_html_with_str_language(
    lang: &str,
    code: &str,
    attr_map: &dyn Fn(&str) -> String,
) -> Option<String> {
    if let Some(l) = languages::LANGUAGES.get(lang) {
        // println!("For {lang}");
        Some(highlight_to_html(l, code, attr_map))
    } else {
        None
    }
}
pub fn highlight_to_html(lang: &Language, code: &str, attr_map: &dyn Fn(&str) -> String) -> String {
    let mut highlighter = Highlighter::new();
    let config = load_language(&lang);

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
