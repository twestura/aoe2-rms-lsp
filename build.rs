//! Build script for aoe2-rms-lsp.
//!
//! ## Predefined pipeline
//! Scans `data/constants/` and `data/labels/` for CSV files and generates
//! Rust source code into `OUT_DIR/constants.rs` and `OUT_DIR/labels.rs`.
//! These are included into the main crate at compile time via the `predefined`
//! module.
//!
//! Constants CSVs have two columns (name, value) and produce `&[(&str, i32)]`
//! slices named `{FILENAME}_CONSTANTS`, e.g. `TERRAIN_CONSTANTS`.
//!
//! Labels CSVs have one column (name) and produce `&[&str]` slices named
//! `{FILENAME}_LABELS`, e.g. `GAME_MODE_LABELS`.
//!
//! ## Hover pipeline
//! Reads `data/hover.toml` and generates `OUT_DIR/hover.rs` containing
//! `lookup_hover(name: &str) -> Option<&'static str>`, with hover doc
//! content embedded as string literals.
//!
//! ## Completions pipeline
//! Reads `data/completions.toml` and generates `OUT_DIR/completions.rs`
//! containing `static COMPLETABLE_TOKENS: &[CompletableToken]`. The TOML
//! section a token belongs to (`[sections]`, `[commands]`, `[attributes]`,
//! `[keywords]`) determines its `CompletionItemKind` in the generated array:
//! ```text
//! CompletableToken { label: "create_land", lower: "create_land", kind: CompletionItemKind::FUNCTION, snippet: Some("create_land {\n\t$0\n}") },
//! ```

use std::{env, ffi::OsStr, fs, path::Path};

/// The kind of predefined data to generate, either constants or labels.
///
/// Constants are name-value pairs loaded from `data/constants/` and produce
/// `&[(&str, i32)]` slices. Labels are name-only values loaded from
/// `data/labels/` and produce `&[&str]` slices.
#[derive(Debug, Clone, Copy)]
enum PredefinedKind {
    Constants,
    Labels,
}

impl PredefinedKind {
    /// The stem used to derive the subdirectory and output filename for this
    /// kind. The subdirectory is `data/{stem}` and the output file is
    /// `{stem}.rs`.
    fn stem(&self) -> &'static str {
        match self {
            Self::Constants => "constants",
            Self::Labels => "labels",
        }
    }

    /// The subdirectory under `data/` containing the CSV files for this kind.
    fn subdir(&self) -> String {
        format!("data/{}", self.stem())
    }

    /// The name of the output file to write into `OUT_DIR` for this kind.
    fn out_file(&self) -> String {
        format!("{}.rs", self.stem())
    }

    /// The suffix appended to each generated slice name for this kind.
    fn suffix(&self) -> &'static str {
        match self {
            Self::Constants => "CONSTANTS",
            Self::Labels => "LABELS",
        }
    }

    /// The Rust type of each slice element for this kind.
    fn slice_type(&self) -> &'static str {
        match self {
            Self::Constants => "(&str, i32)",
            Self::Labels => "&str",
        }
    }

    /// Returns the Rust source for the static slice declaration header.
    ///
    /// # Example
    ///
    /// For `PredefinedKind::Constants` with `file_stem = "terrain"`:
    /// ```text
    /// pub static TERRAIN_CONSTANTS: &[(&str, i32)] = &[
    /// ```
    fn slice_header(&self, file_stem: &str) -> String {
        format!(
            "pub static {}_{}: &[{}] = &[\n",
            file_stem.to_uppercase(),
            self.suffix(),
            self.slice_type(),
        )
    }

    /// Formats a single CSV line as a Rust slice entry.
    ///
    /// # Examples
    ///
    /// For `PredefinedKind::Constants` with `line = "GRASS,0"`:
    /// ```text
    ///     ("GRASS", 0),
    /// ```
    ///
    /// For `PredefinedKind::Labels` with `line = "DEATH_MATCH"`:
    /// ```text
    ///     "DEATH_MATCH",
    /// ```
    fn format_entry(&self, line: &str, path: &str) -> String {
        match self {
            Self::Constants => {
                let (name, value) = line
                    .split_once(',')
                    .unwrap_or_else(|| panic!("Invalid line in {path}: {line:?}"));
                format!("    (\"{}\", {}),\n", name.trim(), value.trim())
            }
            Self::Labels => format!("    \"{line}\",\n"),
        }
    }
}

/// Returns a sorted list of CSV file stems found in the given directory.
fn predefined_file_stems(subdir: &str) -> Vec<String> {
    // Maps every entry in `subdir` to its stem if it has a `.csv` extension.
    let mut file_stems: Vec<String> = fs::read_dir(subdir)
        .unwrap_or_else(|e| panic!("Failed to read directory {subdir}: {e}"))
        .filter_map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("Failed to read entry in {subdir}: {e}"));
            let path = entry.path();
            // If the file has a `.csv` extension, include its stem.
            match path.extension().and_then(OsStr::to_str) {
                Some("csv") => path.file_stem().and_then(OsStr::to_str).map(String::from),
                _ => None,
            }
        })
        .collect();
    file_stems.sort();
    file_stems
}

/// Generates the Rust source for a single CSV file as a static slice
/// declaration and returns it as a `String`.
///
/// For constants, each line must be a `name,value` pair. For labels, each
/// line is a single name. Empty lines are skipped.
fn generate_predefined_slice(path: &str, file_stem: &str, kind: PredefinedKind) -> String {
    let csv = fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {path}: {e}"));

    let mut output = kind.slice_header(file_stem);
    for line in csv.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        output.push_str(&kind.format_entry(line, path));
    }
    // Adds a final closing `];` to account for the opening `[` from `kind.slice_header`.
    output.push_str("];\n\n");
    output
}

/// Scans the directory for this kind, generates a Rust source file containing
/// one static slice per CSV file, and writes it to `OUT_DIR`.
///
/// Registers the directory with `cargo:rerun-if-changed` so the build script
/// is re-run whenever a file is added, removed, or modified.
fn generate_predefined(out_dir: &str, kind: PredefinedKind) {
    let subdir = kind.subdir();
    let dest = Path::new(out_dir).join(kind.out_file());
    println!("cargo:rerun-if-changed={subdir}");
    let filenames = predefined_file_stems(&subdir);

    let output: String = filenames
        .iter()
        .map(|filename| {
            let path = format!("{subdir}/{filename}.csv");
            generate_predefined_slice(&path, filename, kind)
        })
        .collect();

    fs::write(&dest, &output).unwrap_or_else(|e| panic!("Failed to write {dest:?}: {e}"));
}

/// Returns pairs of (name, hover_doc) from the given TOML string.
/// The toml file may contain single-line comments and blank lines, these are
/// ignored.
/// Each line of the toml file has the format:
/// ```toml
/// token_name = "category/filename.md"
/// ```
/// `token_name`s that contain the characters <, >, or # are quoted.
///
/// Comments in the toml file must be on their own line.
fn read_hover_pairs(toml: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    for line in toml.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (name, doc_path) = line
            .split_once('=')
            .unwrap_or_else(|| panic!("Invalid line in data/hover.toml: {line:?}"));
        let name = name.trim().trim_matches('"');
        let doc_path = doc_path.trim().trim_matches('"');
        pairs.push((name.to_string(), doc_path.to_string()));
    }
    pairs
}

/// Turns hover pairs into the Rust source for `hover.rs`, embedding the
/// content of each hover doc file as a string literal in the match arms.
fn hover_pairs_to_rust(pairs: &[(String, String)]) -> String {
    let mut output = String::new();
    output.push_str("/// Returns the hover documentation for the given RMS token, or `None`\n");
    output.push_str("/// if the token has no documentation.\n");
    output.push_str("pub fn lookup_hover(name: &str) -> Option<&'static str> {\n");
    output.push_str("    match name {\n");
    for (name, doc_path) in pairs {
        let path = format!("data/hover_docs/{doc_path}");
        let content =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read {path}: {e}"));
        assert!(
            !content.contains("\"##"),
            "hover doc {path} contains '\"##' which conflicts with the raw string delimiter"
        );
        output.push_str(&format!(
            "        \"{name}\" => Some(r##\"{content}\"##),\n"
        ));
    }
    output.push_str("        _ => None,\n");
    output.push_str("    }\n");
    output.push_str("}\n");
    output
}

/// Generates Rust source files from `data/hover.toml` and writes it
/// to `OUT_DIR/hover.rs`. The output file generates the function
/// ```rust
/// lookup_hover(name: &str) -> Option<&'static str>
/// ```
/// with hover doc content embedded as string literals.
fn generate_hover(out_dir: &str) {
    let dest = Path::new(out_dir).join("hover.rs");
    println!("cargo:rerun-if-changed=data/hover.toml");
    println!("cargo:rerun-if-changed=data/hover_docs/");
    let toml = fs::read_to_string("data/hover.toml")
        .unwrap_or_else(|e| panic!("Failed to read data/hover.toml: {e}"));
    let pairs = read_hover_pairs(&toml);
    let output = hover_pairs_to_rust(&pairs);
    fs::write(&dest, &output).unwrap_or_else(|e| panic!("Failed to write {dest:?}: {e}"));
}

/// A single entry parsed from `data/completions.toml`.
struct CompletionEntry {
    /// The display label shown in the completion popup, with correct casing.
    label: String,
    /// The Rust expression for the `CompletionItemKind` constant, e.g.
    /// `"CompletionItemKind::FUNCTION"`. See:
    /// <https://docs.rs/lsp-types/latest/lsp_types/struct.CompletionItemKind.html>
    kind: &'static str,
    /// The snippet to insert when the completion is accepted, or `None` if
    /// the label should be inserted as-is.
    snippet: Option<String>,
}

impl CompletionEntry {
    /// Returns a line of Rust source representing this entry as a
    /// `CompletableToken` struct literal, suitable for inclusion in the
    /// `COMPLETABLE_TOKENS` static array.
    ///
    /// The `lower` field is derived from `label` via `to_ascii_lowercase`.
    /// A `None` snippet produces `snippet: None`; a `Some(s)` snippet
    /// produces `snippet: Some("s")`.
    ///
    /// # Example
    ///
    /// For an entry with `label = "create_land"`, `kind = "CompletionItemKind::FUNCTION"`,
    /// and `snippet = Some("create_land {\n\t$0\n}")`:
    /// ```text
    ///     CompletableToken { label: "create_land", lower: "create_land", kind: CompletionItemKind::FUNCTION, snippet: Some("create_land {\n\t$0\n}") },
    /// ```
    fn to_rust(&self) -> String {
        let label = &self.label;
        let lower = self.label.to_ascii_lowercase();
        let kind = self.kind;
        let snippet = match &self.snippet {
            None => "None".to_string(),
            Some(s) => format!("Some(\"{s}\")"),
        };
        format!(
            "    CompletableToken {{ label: \"{label}\", lower: \"{lower}\", kind: {kind}, snippet: {snippet} }},\n",
        )
    }
}

/// Returns entries parsed from the given completions TOML string.
///
/// Section headers (`[sections]`, `[commands]`, etc.) determine the
/// `CompletionItemKind` for the entries that follow. Each entry line has the
/// format:
/// ```toml
/// token_name = "snippet"
/// ```
/// An empty string value (`""`) means no snippet; the label is inserted as-is
/// (`snippet: None` in the generated code).
/// Keys containing `<`, `>`, or `#` are quoted.
///
/// Comments and blank lines are ignored. Comments must be on their own line.
fn read_completion_entries(toml: &str) -> Vec<CompletionEntry> {
    let mut entries = vec![];
    let mut current_kind = None;
    for line in toml.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parses a section header, setting the current kind for entries that follow.
        if line.starts_with('[') && line.ends_with(']') {
            let section = &line[1..line.len() - 1];
            current_kind = Some(match section {
                "sections" => "CompletionItemKind::MODULE",
                "commands" => "CompletionItemKind::FUNCTION",
                "attributes" => "CompletionItemKind::PROPERTY",
                "keywords" => "CompletionItemKind::KEYWORD",
                _ => panic!("Unknown section in data/completions.toml: {section:?}"),
            });
            continue;
        }

        // Parses a key-value pair, adding it to the entries with the current kind.
        let kind = current_kind.unwrap_or_else(|| {
            panic!("Entry before any section in data/completions.toml: {line:?}")
        });
        let (key, value) = line
            .split_once('=')
            .unwrap_or_else(|| panic!("Invalid line in data/completions.toml: {line:?}"));
        let label = key.trim().trim_matches('"').to_string();
        let snippet = match value.trim().trim_matches('"') {
            "" => None,
            s => Some(s.to_string()),
        };
        entries.push(CompletionEntry {
            label,
            kind,
            snippet,
        });
    }
    entries
}

/// Turns completion entries into the Rust source for `completions.rs`,
/// producing a `static COMPLETABLE_TOKENS` array with one entry per token.
///
/// Note: snippet strings containing `"` are not supported and will produce
/// invalid Rust source. None of the current snippets contain double quotation
/// marks.
fn completion_entries_to_rust(entries: &[CompletionEntry]) -> String {
    let mut output = String::new();
    output.push_str("/// All tokens that are recognized by the language server and can be offered as\n");
    output.push_str("/// completion suggestions, along with their lowercase forms for case-insensitive\n");
    output.push_str("/// matching, their kinds for coloring and icons in the completion popup, and\n");
    output.push_str("/// optional snippets for expanding to full syntax with placeholders.\n");
    output.push_str("static COMPLETABLE_TOKENS: &[CompletableToken] = &[\n");
    output.extend(entries.iter().map(CompletionEntry::to_rust));
    output.push_str("];\n");
    output
}

/// Generates Rust source from `data/completions.toml` and writes it to
/// `OUT_DIR/completions.rs`. The output file defines:
/// ```rust
/// static COMPLETABLE_TOKENS: &[CompletableToken]
/// ```
fn generate_completions(out_dir: &str) {
    let dest = Path::new(out_dir).join("completions.rs");
    println!("cargo:rerun-if-changed=data/completions.toml");
    let toml = fs::read_to_string("data/completions.toml")
        .unwrap_or_else(|e| panic!("Failed to read data/completions.toml: {e}"));
    let entries = read_completion_entries(&toml);
    let output = completion_entries_to_rust(&entries);
    fs::write(&dest, &output).unwrap_or_else(|e| panic!("Failed to write {dest:?}: {e}"));
}

/// Generates Rust source files from all data files in `data/`.
fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    generate_predefined(&out_dir, PredefinedKind::Constants);
    generate_predefined(&out_dir, PredefinedKind::Labels);
    generate_hover(&out_dir);
    generate_completions(&out_dir);
}
