use regex::Regex;
use std::{collections::HashMap, sync::RwLock};

use phf::phf_map;

pub static HIGHLIGHTS: phf::Map<&'static str, &'static str> = phf_map! {
    "rust" => include_str!("../queries/rust/highlights.scm"),
    "javascript" => include_str!("../queries/javascript/highlights.scm"),
    "jsx" => include_str!("../queries/jsx/highlights.scm"),
    "typescript" => include_str!("../queries/typescript/highlights.scm"),
    "tsx" => include_str!("../queries/tsx/highlights.scm"),
    "python" => include_str!("../queries/python/highlights.scm"),
    "cpp" => include_str!("../queries/cpp/highlights.scm"),
    "c" => include_str!("../queries/c/highlights.scm"),
    // "java" => include_str!("../queries/java/highlights.scm"),
    "java" => include_str!("../languages/tree-sitter-java/queries/highlights.scm"),
    "go" => include_str!("../queries/go/highlights.scm"),
};

pub static INJECTIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "rust" => include_str!("../queries/rust/injections.scm"),
    "javascript" => include_str!("../queries/javascript/injections.scm"),
    "typescript" => include_str!("../queries/typescript/injections.scm"),
    "tsx" => include_str!("../queries/tsx/injections.scm"),
    "python" => include_str!("../queries/python/injections.scm"),
    "cpp" => include_str!("../queries/cpp/injections.scm"),
    "c" => include_str!("../queries/c/injections.scm"),
    "java" => include_str!("../queries/java/injections.scm"),
    "go" => include_str!("../queries/go/injections.scm"),
};

pub static LOCALS: phf::Map<&'static str, &'static str> = phf_map! {
    "rust" => include_str!("../queries/rust/locals.scm"),
    "javascript" => include_str!("../queries/javascript/locals.scm"),
    "typescript" => include_str!("../queries/typescript/locals.scm"),
    "tsx" => include_str!("../queries/tsx/locals.scm"),
    "python" => include_str!("../queries/python/locals.scm"),
    "cpp" => include_str!("../queries/cpp/locals.scm"),
    "c" => include_str!("../queries/c/locals.scm"),
    "java" => include_str!("../queries/java/locals.scm"),
    "go" => include_str!("../queries/go/locals.scm"),
};

lazy_static::lazy_static! {
    static ref INHERITS_RE: Regex = Regex::new(r"^;+\s*inherits\s*:?\s*([a-z_,()]+)\s*$").unwrap();
}

// TODO: recursion detection
pub fn get_query(map: &phf::Map<&'static str, &'static str>, lang: &str) -> Option<String> {
    if let Some(s) = map.get(lang) {
        let s = *s;
        if let Some(first_line_pos) = s.find("\n") {
            let first_line = &s[..first_line_pos];
            if let Some(caps) = (&*INHERITS_RE).captures(first_line) {
                if let Some(m) = caps.get(1) {
                    let mut r = s.to_owned();
                    for l in m.as_str().split(",") {
                        if let Some(q) = get_query(map, l) {
                            r.push_str(&q);
                        } else {
                            return None
                        }
                    }

                    Some(r)
                } else {
                    Some(s.to_owned())
                }
            } else {
                Some(s.to_owned())
            }
        } else {
            Some(s.to_owned())
        }
    } else {
        None
    }
}
