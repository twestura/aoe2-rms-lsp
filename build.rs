//! Build script for aoe2-rms-lsp.
//!
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

use std::{env, ffi::OsStr, fs, path::Path};

/// The kind of predefined data to generate, either constants or labels.
///
/// Constants are name-value pairs loaded from `data/constants/` and produce
/// `&[(&str, i32)]` slices. Labels are name-only values loaded from
/// `data/labels/` and produce `&[&str]` slices.
#[derive(Debug, Clone, Copy)]
enum GenerateKind {
    Constants,
    Labels,
}

impl GenerateKind {
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
    /// For `GenerateKind::Constants` with `file_stem = "terrain"`:
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
    /// For `GenerateKind::Constants` with `line = "GRASS,0"`:
    /// ```text
    ///     ("GRASS", 0),
    /// ```
    ///
    /// For `GenerateKind::Labels` with `line = "DEATH_MATCH"`:
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
fn csv_file_stems(subdir: &str) -> Vec<String> {
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
fn generate_slice(path: &str, file_stem: &str, kind: GenerateKind) -> String {
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
fn generate(out_dir: &str, kind: GenerateKind) {
    let subdir = kind.subdir();
    let dest = Path::new(out_dir).join(kind.out_file());
    println!("cargo:rerun-if-changed={subdir}");
    let filenames = csv_file_stems(&subdir);

    let output: String = filenames
        .iter()
        .map(|filename| {
            let path = format!("{subdir}/{filename}.csv");
            generate_slice(&path, filename, kind)
        })
        .collect();

    fs::write(&dest, &output).unwrap_or_else(|e| panic!("Failed to write {dest:?}: {e}"));
}

/// Generates Rust source files from the CSV data files in `data/constants/`
/// and `data/labels/`.
fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    generate(&out_dir, GenerateKind::Constants);
    generate(&out_dir, GenerateKind::Labels);
}
