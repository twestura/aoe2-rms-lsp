//! Parses `data/language.toml` and generates `language_data.rs`.
//!
//! ## Output
//!
//! `language_data.rs` contains two items:
//!
//! 1. `pub fn lookup_hover(name: &str) -> Option<&'static str>` — a match
//!    function with one arm per token that has a `hover` field. The hover doc
//!    content is read from `data/hover_docs/<path>` and embedded as a raw
//!    string literal (`r##"..."##`), identical to the format produced by the
//!    existing `hover.rs` pipeline.
//!
//! 2. `static COMPLETABLE_TOKENS: &[CompletableToken]` — a flat array of
//!    completion entries in the same format as the existing `completions.rs`
//!    pipeline. `command_attribute` tokens appear twice: first with
//!    `CompletionItemKind::FUNCTION` (command position), then with
//!    `CompletionItemKind::PROPERTY` (attribute position).
//!
//! ## Snippet derivation
//!
//! Snippets are derived from each token's fields unless an explicit `snippet`
//! field is present in `language.toml`, which takes precedence. The derivation
//! rules are:
//!
//! - **Sections** — always `snippet: None`; inserted as-is.
//! - **Commands / command_attributes as FUNCTION** — if `block = true`, a
//!   ` {\n\t$0\n}` suffix is appended after any inline arg placeholders.
//!   Inline args, if any, are formatted as `${N:NAME}` placeholders. If
//!   neither args nor block, `snippet: None`.
//! - **Attributes / command_attributes as PROPERTY** — arg-list only:
//!   `${N:NAME}` placeholders joined by spaces. `snippet: None` if no args.
//! - **Keywords** — same as attributes unless an explicit `snippet` is set.
//!
//! ## Token type → CompletionItemKind
//!
//! | Token type        | Label format | Kind       |
//! |-------------------|--------------|------------|
//! | section           | `<NAME>`     | MODULE     |
//! | command           | `name`       | FUNCTION   |
//! | attribute         | `name`       | PROPERTY   |
//! | command_attribute | `name`       | FUNCTION + PROPERTY (two entries) |
//! | keyword           | `name`       | KEYWORD    |
//!
//! ## Ordering
//!
//! All five token-type tables are deserialized into `BTreeMap`, which iterates
//! in alphabetical key order. This ensures the generated `language_data.rs` is
//! sorted consistently on every build, producing a stable file regardless of
//! Rust's hash-map randomisation.

use std::{collections::BTreeMap, fs, path::Path};

/// The top-level structure of `data/language.toml`, with one table per token
/// type. Each table maps token names to their entry data. The `BTreeMap` type
/// is used deliberately to produce alphabetically sorted output on every build.
#[derive(serde::Deserialize)]
struct LanguageFile {
    /// Section header tokens (e.g. `<LAND_GENERATION>`). Keys are written
    /// without angle brackets (e.g. `LAND_GENERATION`).
    section: BTreeMap<String, Token>,
    /// Top-level command tokens valid inside a section block.
    command: BTreeMap<String, Token>,
    /// Attribute tokens valid inside a command block.
    attribute: BTreeMap<String, Token>,
    /// Tokens valid as both a command and an attribute depending on position.
    command_attribute: BTreeMap<String, Token>,
    /// Preprocessor keyword tokens (e.g. `if`, `#define`, `start_random`).
    keyword: BTreeMap<String, Token>,
}

/// Data for a single token in `language.toml`.
#[derive(serde::Deserialize)]
struct Token {
    /// Token ID from the RMS Equivalencies spreadsheet. Absent for tokens not
    /// present in the spreadsheet.
    id: Option<u32>,
    /// Path to the hover doc, relative to `data/hover_docs/`. Every token has
    /// one.
    hover: String,
    /// Sections in which this token is valid. Present on `command` and
    /// `command_attribute` entries.
    sections: Option<Vec<String>>,
    /// Commands inside which this token is valid as an attribute. Present on
    /// `attribute` and `command_attribute` entries.
    commands: Option<Vec<String>>,
    /// If `true`, this token may appear more than once in its context.
    repeatable: Option<bool>,
    /// If `true`, this command accepts a `{ }` block body in RMS syntax.
    /// Only present on `command` and `command_attribute` entries.
    block: Option<bool>,
    /// Explicit snippet override. Used for keywords whose snippets cannot be
    /// derived from `args` alone (e.g. `if`/`endif` pairs).
    snippet: Option<String>,
    /// Ordered list of arguments, used to derive `${N:NAME}` tab stops.
    args: Option<Vec<Argument>>,
}

impl Token {
    /// Derives the completion snippet for this token, ready to embed in a
    /// generated Rust string literal.
    ///
    /// If an explicit `snippet` field is set it is used as the base; otherwise
    /// the snippet is derived from `args` and `block`:
    ///
    /// - If neither args nor block are present, returns `None`.
    /// - Arguments, if any, are formatted as `${N:NAME}` tab stops joined by spaces.
    /// - If `block` is `true`, ` {\n\t$0\n}` is appended after any args.
    ///
    /// The returned string is escaped for embedding inside a Rust `"..."` double-
    /// quoted string literal. The `toml` crate decodes TOML escape sequences
    /// (e.g. `\n`, `\t` in snippet fields) into actual characters; this method
    /// re-escapes them. Escape order: `\` → `\\`, then `"` → `\"`, then newline
    /// → `\n`, then tab → `\t`. No other special characters appear in the current
    /// snippet data (`<`, `>`, and `#` are not special in Rust string literals).
    ///
    /// `name` is the token's map key, needed to prefix the snippet string.
    fn snippet(&self, name: &str) -> Option<String> {
        let raw = if let Some(explicit) = &self.snippet {
            explicit.clone()
        } else {
            let has_block = self.block.unwrap_or(false);
            let args = self.args.as_deref().unwrap_or(&[]);
            if args.is_empty() && !has_block {
                return None;
            }
            let mut parts = vec![name.to_string()];
            for (i, arg) in args.iter().enumerate() {
                parts.push(format!("${{{n}:{name}}}", n = i + 1, name = arg.name));
            }
            if has_block {
                parts.push("{\n\t$0\n}".to_string());
            }
            parts.join(" ")
        };
        // Escape for embedding in a Rust double-quoted string literal.
        // Order matters: backslash first to avoid double-escaping.
        Some(
            raw.replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\t', "\\t"),
        )
    }
}

/// A single argument in a token's `args` array.
#[derive(serde::Deserialize)]
struct Argument {
    /// Display name of the argument, used as the tab-stop placeholder text in
    /// snippets (e.g. `TERRAIN` → `${1:TERRAIN}`).
    name: String,
    /// Semantic kind of the argument (e.g. `"terrain"`, `"number"`).
    kind: String,
    /// If `true`, this argument may be omitted at the call site.
    optional: Option<bool>,
}

/// Generates `OUT_DIR/hover.rs` and `OUT_DIR/completions.rs` from
/// `data/language.toml`.
///
/// `hover.rs` defines:
/// ```rust
/// pub fn lookup_hover(name: &str) -> Option<&'static str>
/// ```
/// `completions.rs` defines:
/// ```rust
/// static COMPLETABLE_TOKENS: &[CompletableToken]
/// ```
/// These filenames and APIs are identical to those produced by the old
/// `hover.toml` and `completions.toml` pipelines, so the `src/` crate
/// requires no changes.
pub fn generate(out_dir: &str) {
    let lang = read_language_toml();
    let hover_dest = Path::new(out_dir).join("hover.rs");
    fs::write(&hover_dest, generate_hover_src(&lang))
        .unwrap_or_else(|e| panic!("Failed to write {hover_dest:?}: {e}"));
    let completions_dest = Path::new(out_dir).join("completions.rs");
    fs::write(&completions_dest, generate_completions_src(&lang))
        .unwrap_or_else(|e| panic!("Failed to write {completions_dest:?}: {e}"));
}

/// Reads `data/language.toml` and deserializes it into a [`LanguageFile`].
///
/// Panics if the file cannot be read or if the TOML does not match the
/// expected structure.
fn read_language_toml() -> LanguageFile {
    let src = fs::read_to_string("data/language.toml")
        .unwrap_or_else(|e| panic!("Failed to read data/language.toml: {e}"));
    toml::from_str(&src).unwrap_or_else(|e| panic!("Failed to parse data/language.toml: {e}"))
}

/// Turns a [`LanguageFile`] into the Rust source for the `lookup_hover` match
/// function, embedding the content of each hover doc file from
/// `data/hover_docs/` as a raw string literal (`r##"..."##`).
///
/// All five token-type tables are covered. Within each table, tokens are
/// visited in alphabetical order (guaranteed by [`BTreeMap`]).
///
/// Panics if a hover doc file cannot be read, or if its content contains
/// `"##`, which would conflict with the raw string delimiter.
fn generate_hover_src(lang: &LanguageFile) -> String {
    let mut output = String::new();
    output.push_str("/// Returns the hover documentation for the given RMS token, or `None`\n");
    output.push_str("/// if the token has no documentation.\n");
    output.push_str("pub fn lookup_hover(name: &str) -> Option<&'static str> {\n");
    output.push_str("    match name {\n");
    // Section tokens use `<NAME>` as their token text, so the match key includes angle brackets.
    for (name, token) in &lang.section {
        let key = format!("<{name}>");
        output.push_str(&hover_arm(&key, &token.hover));
    }
    for table in [
        &lang.command,
        &lang.attribute,
        &lang.command_attribute,
        &lang.keyword,
    ] {
        for (name, token) in table {
            output.push_str(&hover_arm(name, &token.hover));
        }
    }
    output.push_str("        _ => None,\n");
    output.push_str("    }\n");
    output.push_str("}\n");
    output
}

/// Generates a single match arm: `"key" => Some(r##"...content..."##),`.
fn hover_arm(key: &str, hover_path: &str) -> String {
    let path = format!("data/hover_docs/{hover_path}");
    let content =
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read {path}: {e}"));
    assert!(
        !content.contains("\"##"),
        "hover doc {path} contains '\"##' which conflicts with the raw string delimiter"
    );
    format!("        \"{key}\" => Some(r##\"{content}\"##),\n")
}

/// Turns a [`LanguageFile`] into the Rust source for the `COMPLETABLE_TOKENS`
/// static array, producing one [`CompletableToken`] struct literal per entry.
///
/// Tokens are emitted in this order: sections (`MODULE`), commands
/// (`FUNCTION`), command_attributes (`FUNCTION`), attributes (`PROPERTY`),
/// command_attributes (`PROPERTY`), keywords (`KEYWORD`). Within each group,
/// tokens appear in alphabetical order. `command_attribute` tokens are emitted
/// twice — once as `FUNCTION` and once as `PROPERTY` — because they are valid
/// in both positions.
///
/// Section labels are wrapped in angle brackets (e.g. `<PLAYER_SETUP>`).
/// Snippets are derived by [`Token::snippet`].
fn generate_completions_src(lang: &LanguageFile) -> String {
    let mut output = String::new();
    output.push_str(
        "/// All tokens that are recognized by the language server and can be offered as\n",
    );
    output.push_str(
        "/// completion suggestions, along with their lowercase forms for case-insensitive\n",
    );
    output.push_str(
        "/// matching, their kinds for coloring and icons in the completion popup, and\n",
    );
    output.push_str("/// optional snippets for expanding to full syntax with placeholders.\n");
    output.push_str("static COMPLETABLE_TOKENS: &[CompletableToken] = &[\n");
    for (name, _) in &lang.section {
        let label = format!("<{name}>");
        output.push_str(&completion_entry_to_rust(
            &label,
            "CompletionItemKind::MODULE",
            None,
        ));
    }
    for (name, token) in &lang.command {
        output.push_str(&completion_entry_to_rust(
            name,
            "CompletionItemKind::FUNCTION",
            token.snippet(name),
        ));
    }
    for (name, token) in &lang.command_attribute {
        output.push_str(&completion_entry_to_rust(
            name,
            "CompletionItemKind::FUNCTION",
            token.snippet(name),
        ));
    }
    for (name, token) in &lang.attribute {
        output.push_str(&completion_entry_to_rust(
            name,
            "CompletionItemKind::PROPERTY",
            token.snippet(name),
        ));
    }
    for (name, token) in &lang.command_attribute {
        output.push_str(&completion_entry_to_rust(
            name,
            "CompletionItemKind::PROPERTY",
            token.snippet(name),
        ));
    }
    for (name, token) in &lang.keyword {
        output.push_str(&completion_entry_to_rust(
            name,
            "CompletionItemKind::KEYWORD",
            token.snippet(name),
        ));
    }
    output.push_str("];\n");
    output
}

/// Returns a line of Rust source representing one entry in the
/// `COMPLETABLE_TOKENS` static array as a `CompletableToken` struct literal.
///
/// The `lower` field is derived from `label` via `to_ascii_lowercase`.
/// A `None` snippet produces `snippet: None`; a `Some(s)` snippet produces
/// `snippet: Some("s")`, where `s` is already escaped for a Rust string
/// literal (see [`Token::snippet`]).
///
/// # Example
///
/// For `label = "create_land"`, `kind = "CompletionItemKind::FUNCTION"`,
/// and `snippet = Some("create_land {\\n\\t$0\\n}")`:
/// ```text
///     CompletableToken { label: "create_land", lower: "create_land", kind: CompletionItemKind::FUNCTION, snippet: Some("create_land {\n\t$0\n}") },
/// ```
fn completion_entry_to_rust(label: &str, kind: &str, snippet: Option<String>) -> String {
    let lower = label.to_ascii_lowercase();
    let snippet_src = match snippet {
        None => "None".to_string(),
        Some(s) => format!("Some(\"{s}\")"),
    };
    format!(
        "    CompletableToken {{ label: \"{label}\", lower: \"{lower}\", kind: {kind}, snippet: {snippet_src} }},\n"
    )
}
