//! Smoke tests for the `hover.rs` and `completions.rs` files generated from
//! `data/language.toml`.
//!
//! Tests are divided into three groups:
//!
//! - **Hover** — checks that a `lookup_hover` match arm is present for one
//!   representative token per type (section, command, attribute,
//!   command_attribute, keyword). Catches accidental omission of an entire
//!   token category from the hover output.
//!
//! - **Completions** — checks that a `COMPLETABLE_TOKENS` entry is present for
//!   one representative token per type. Catches accidental omission of an
//!   entire token category from the completions output.
//!
//! - **Full entries** — checks the complete generated struct literal for a
//!   selection of tokens, covering the four distinct snippet patterns: no
//!   snippet (section), derived block-body snippet (command), derived arg-list
//!   snippet (attribute), and explicit snippet (keyword).

/// The full text of the generated `hover.rs` file, embedded at compile time
/// via `OUT_DIR`. Tests perform plain substring searches against this string
/// rather than parsing or compiling it.
const HOVER_SRC: &str = include_str!(concat!(env!("OUT_DIR"), "/hover.rs"));

/// The full text of the generated `completions.rs` file, embedded at compile
/// time via `OUT_DIR`. Tests perform plain substring searches against this
/// string rather than parsing or compiling it.
const COMPLETIONS_SRC: &str = include_str!(concat!(env!("OUT_DIR"), "/completions.rs"));

// ---------------------------------------------------------------------------
// Hover — one representative per token type
// ---------------------------------------------------------------------------

/// Checks that the `lookup_hover` function contains an arm for the
/// `PLAYER_SETUP` section token.
#[test]
fn hover_section() {
    assert!(HOVER_SRC.contains("\"<PLAYER_SETUP>\" => Some("), "missing hover for section <PLAYER_SETUP>");
}

/// Checks that the `lookup_hover` function contains an arm for the
/// `random_placement` command token.
#[test]
fn hover_command() {
    assert!(
        HOVER_SRC.contains("\"random_placement\" => Some("),
        "missing hover for command random_placement"
    );
}

/// Checks that the `lookup_hover` function contains an arm for the
/// `terrain_type` attribute token.
#[test]
fn hover_attribute() {
    assert!(
        HOVER_SRC.contains("\"terrain_type\" => Some("),
        "missing hover for attribute terrain_type"
    );
}

/// Checks that the `lookup_hover` function contains an arm for the
/// `base_terrain` command_attribute token.
#[test]
fn hover_command_attribute() {
    assert!(
        HOVER_SRC.contains("\"base_terrain\" => Some("),
        "missing hover for command_attribute base_terrain"
    );
}

/// Checks that the `lookup_hover` function contains an arm for the
/// `if` keyword token.
#[test]
fn hover_keyword() {
    assert!(HOVER_SRC.contains("\"if\" => Some("), "missing hover for keyword if");
}

// ---------------------------------------------------------------------------
// Completions — one representative per token type
// ---------------------------------------------------------------------------

/// Checks that `COMPLETABLE_TOKENS` contains an entry for the `<PLAYER_SETUP>`
/// section token.
#[test]
fn completion_section() {
    assert!(
        COMPLETIONS_SRC.contains("label: \"<PLAYER_SETUP>\""),
        "missing completion for section <PLAYER_SETUP>"
    );
}

/// Checks that `COMPLETABLE_TOKENS` contains an entry for the `create_land`
/// command token.
#[test]
fn completion_command() {
    assert!(
        COMPLETIONS_SRC.contains("label: \"create_land\""),
        "missing completion for command create_land"
    );
}

/// Checks that `COMPLETABLE_TOKENS` contains an entry for the `terrain_type`
/// attribute token.
#[test]
fn completion_attribute() {
    assert!(
        COMPLETIONS_SRC.contains("label: \"terrain_type\""),
        "missing completion for attribute terrain_type"
    );
}

/// Checks that `COMPLETABLE_TOKENS` contains an entry for the `base_terrain`
/// command_attribute token. This token appears twice in the array — once with
/// `CompletionItemKind::FUNCTION` (command position) and once with
/// `CompletionItemKind::PROPERTY` (attribute position).
#[test]
fn completion_command_attribute() {
    assert!(
        COMPLETIONS_SRC.contains("label: \"base_terrain\""),
        "missing completion for command_attribute base_terrain"
    );
}

/// Checks that `COMPLETABLE_TOKENS` contains an entry for the `if` keyword
/// token.
#[test]
fn completion_keyword() {
    assert!(COMPLETIONS_SRC.contains("label: \"if\""), "missing completion for keyword if");
}

// ---------------------------------------------------------------------------
// Full-entry checks — verify kind, lower, and snippet together
// ---------------------------------------------------------------------------

/// Checks the complete `CompletableToken` literal for the `<PLAYER_SETUP>`
/// section: kind `MODULE`, lowercased label with angle brackets, and no
/// snippet (sections are inserted as-is).
#[test]
fn full_entry_section_player_setup() {
    assert!(
        COMPLETIONS_SRC.contains(
            r#"CompletableToken { label: "<PLAYER_SETUP>", lower: "<player_setup>", kind: CompletionItemKind::MODULE, snippet: None },"#
        ),
        "unexpected full entry for section <PLAYER_SETUP>"
    );
}

/// Checks the complete `CompletableToken` literal for the `create_land`
/// command: kind `FUNCTION` and a derived block-body snippet. `create_land`
/// takes no inline args but accepts attributes inside a `{ }` block, so the
/// snippet is derived as `create_land {\n\t$0\n}`.
#[test]
fn full_entry_command_create_land() {
    assert!(
        COMPLETIONS_SRC.contains(
            r#"CompletableToken { label: "create_land", lower: "create_land", kind: CompletionItemKind::FUNCTION, snippet: Some("create_land {\n\t$0\n}") },"#
        ),
        "unexpected full entry for command create_land"
    );
}

/// Checks the complete `CompletableToken` literal for the `if` keyword: kind
/// `KEYWORD` and an explicit snippet (stored directly in `language.toml`
/// because the closing `endif` cannot be derived from the `args` field alone).
#[test]
fn full_entry_keyword_if() {
    assert!(
        COMPLETIONS_SRC.contains(
            r#"CompletableToken { label: "if", lower: "if", kind: CompletionItemKind::KEYWORD, snippet: Some("if ${1:CONDITION}\n\t$0\nendif") },"#
        ),
        "unexpected full entry for keyword if"
    );
}

/// Checks the complete `CompletableToken` literal for the `terrain_type`
/// attribute: kind `PROPERTY` and a derived arg-list snippet. `terrain_type`
/// takes one `TERRAIN` argument, so the snippet is derived as
/// `terrain_type ${1:TERRAIN}`.
#[test]
fn full_entry_attribute_terrain_type() {
    assert!(
        COMPLETIONS_SRC.contains(
            r#"CompletableToken { label: "terrain_type", lower: "terrain_type", kind: CompletionItemKind::PROPERTY, snippet: Some("terrain_type ${1:TERRAIN}") },"#
        ),
        "unexpected full entry for attribute terrain_type"
    );
}
