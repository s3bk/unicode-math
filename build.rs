use regex::Regex;
use std::path::PathBuf;
use std::{env, fs};
use std::fmt::Write;

 fn atom_from_tex(s: &str) -> &'static str {
    match s {
        "mathalpha" => "Alpha",
        "mathpunct" => "Punctuation",
        "mathopen" => "Open",
        "mathclose" => "Close",
        "mathord" => "Ordinal",
        "mathbin" => "Binary",
        "mathrel" => "Relation",
        "mathop" => "Operator",
        "mathfence" => "Fence",
        "mathover" => "Over",
        "mathunder" => "Under",
        "mathaccent" => "Accent",
        "mathaccentwide" => "AccentWide",
        "mathaccentoverlay" => "AccentOverlay",
        "mathbotaccent" => "BotAccent",
        "mathbotaccentwide" => "BotAccentWide",
        op => panic!("unexpected {:?}", op)
    }
}

fn main() {
    let path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("src").join("unicode-math-table.tex");
    let source = String::from_utf8(fs::read(&path).unwrap()).unwrap();
    let mut out = String::new();

    let re = Regex::new(r#"\\UnicodeMathSymbol\{"([[:xdigit:]]+)\}\{\\([[:alpha:]]+)\s*\}\{\\([[:alpha:]]+)\}\{([^\}]*)\}%"#).unwrap();
    writeln!(out, "[");
    for line in source.lines() {
        if let Some(c) = re.captures(line) {
            dbg!(&c);
            writeln!(out, r"    Symbol {{ codepoint: '\u{{{}}}', name: {:?}, kind: AtomType::{}, description: {:?} }},", &c[1], &c[2], atom_from_tex(&c[3]), &c[4]);
        }
    }
    writeln!(out, "]");

    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("symbols.rs");
    fs::write(out_path, out.as_bytes()).unwrap();
}
