use tree_sitter::Language as TSLanguage;

extern "C" {
    fn tree_sitter_cpp() -> TSLanguage;
    fn tree_sitter_go() -> TSLanguage;
    fn tree_sitter_java() -> TSLanguage;
    fn tree_sitter_python() -> TSLanguage;
    fn tree_sitter_rust() -> TSLanguage;
    fn tree_sitter_javascript() -> TSLanguage;
    fn tree_sitter_typescript() -> TSLanguage;
    fn tree_sitter_tsx() -> TSLanguage;
}

/// Returns the tree-sitter [TSLanguage][] for this grammar.
///
/// [TSLanguage]: https://docs.rs/tree-sitter/*/tree_sitter/struct.TSLanguage.html
pub fn cpp() -> TSLanguage {
    unsafe { tree_sitter_cpp() }
}

pub fn go() -> TSLanguage {
    unsafe { tree_sitter_go() }
}

/// Returns the tree-sitter [TSLanguage][] for this grammar.
///
/// [TSLanguage]: https://docs.rs/tree-sitter/*/tree_sitter/struct.TSLanguage.html
pub fn java() -> TSLanguage {
    unsafe { tree_sitter_java() }
}

pub fn python() -> TSLanguage {
    unsafe { tree_sitter_python() }
}

pub fn rust() -> TSLanguage {
    unsafe { tree_sitter_rust() }
}

pub fn javascript() -> TSLanguage {
    unsafe { tree_sitter_javascript() }
}

pub fn typescript() -> TSLanguage {
    unsafe { tree_sitter_typescript() }
}

/// Returns the tree-sitter [Language][] for TSX.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn tsx() -> TSLanguage {
    unsafe { tree_sitter_tsx() }
}

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
    Go,
}

use phf::phf_map;

pub static LANGUAGES: phf::Map<&'static str, Language> = phf_map! {
    "rust" => Language::Rust,
    "js"   => Language::Javascript,
    "javascript" => Language::Javascript,
    "jsx" => Language::JavascriptJsx,
    "typescript" => Language::Typescript,
    "ts" => Language::Typescript,
    "tsx" => Language::TypescriptTsx,
    "python" => Language::Python,
    "cpp" => Language::Cpp,
    "java" => Language::Java,
    "go" => Language::Go,
};
