use regex::Regex;
use std::path::PathBuf;
use std::{env, fs};
use std::fmt::Write;

const OPERATOR_LIMITS: &'static [&'static str] = &[
    "coprod",
    "bigvee",
    "bigwedge",
    "biguplus",
    "bigcap",
    "bigcup",
    "prod",
    "sum",
    "bigotimes",
    "bigoplus",
    "bigodot",
    "bigsqcup",
];

fn atom_from_tex(name: &str, kind: &str) -> &'static str {
    match kind {
        "mathalpha" => "Alpha",
        "mathpunct" => "Punctuation",
        "mathopen" => "Open",
        "mathclose" => "Close",
        "mathord" => "Ordinal",
        "mathbin" => "Binary",
        "mathrel" => "Relation",
        "mathop" if OPERATOR_LIMITS.contains(&name) => "Operator(true)",
        "mathop" => "Operator(false)",
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
    println!("cargo:rerun-if-changed=src/unicode-math-table.tex");

    let path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("src").join("unicode-math-table.tex");
    let source = String::from_utf8(fs::read(&path).unwrap()).unwrap();
    let mut out = String::new();

    let re = Regex::new(r#"\\UnicodeMathSymbol\{"([[:xdigit:]]+)\}\{\\([[:alpha:]]+)\s*\}\{\\([[:alpha:]]+)\}\{([^\}]*)\}%"#).unwrap();
    writeln!(out, "[").unwrap();
    for line in source.lines() {
        if let Some(c) = re.captures(line) {
            writeln!(out,
                r"    Symbol {{ codepoint: '\u{{{}}}', name: {:?}, kind: AtomType::{}, description: {:?} }},",
                &c[1], &c[2], atom_from_tex(&c[2], &c[3]), &c[4]
            ).unwrap();
        }
    }
    writeln!(out, "]").unwrap();

    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("symbols.rs");
    fs::write(out_path, out.as_bytes()).unwrap();
}
