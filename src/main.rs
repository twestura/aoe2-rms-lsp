//! Aoe2 RMS Language Server

use std::collections::HashMap;

use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, Hover,
    HoverContents, HoverParams, HoverProviderCapability, MarkupContent, MarkupKind, Position,
    PositionEncodingKind, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
    Url,
};
use tower_lsp::{
    Client, LanguageServer, LspService, Server,
    lsp_types::{InitializeParams, InitializeResult, InitializedParams, MessageType},
};

#[derive(Debug)]
struct Backend {
    client: Client,
    /// The server's in-memory document store.
    /// Maps a document's URI to its contents.
    documents: RwLock<HashMap<Url, String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                position_encoding: Some(PositionEncodingKind::UTF8),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
        log("server initialized!");
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.documents.write().await.insert(uri, text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        // For now we use the full text document sync kind that returns
        // the entire document in the change. This is fine for small map
        // scripts (many maps are just a few hundred lines) during development.
        let text = params.content_changes.into_iter().next().unwrap().text;
        self.documents.write().await.insert(uri, text);
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.write().await.remove(&uri);
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let documents = self.documents.read().await;
        let hover_result = documents
            .get(&uri)
            .and_then(|text| extract_token(text, position))
            .and_then(lookup_hover)
            .map(str::to_string)
            .map(|value| Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                range: None,
            });
        Ok(hover_result)
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: RwLock::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}

/// Debugging function for logging messages from the LSP to a file.
fn log(msg: &str) {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("aoe2-rms-lsp.txt")
        .unwrap();
    writeln!(file, "{msg}").unwrap();
}

/// Extracts the token at the given position from the text.
/// Returns `Some(token)` if the position is within text.
/// Otherwise returns `None` if the position is within whitespace.
fn extract_token(text: &str, position: Position) -> Option<&str> {
    let line = text.split("\n").nth(position.line as usize)?;
    let col = position.character as usize;
    let right = line[col..]
        .find(char::is_whitespace)
        .map(|i| i + col)
        .unwrap_or(line.len());
    if right == col {
        return None;
    }
    let left = line[..col]
        .rfind(char::is_whitespace)
        .map(|i| i + 1)
        .unwrap_or(0);
    debug_assert_ne!(
        left, right,
        "right > left because the first character is not whitespace"
    );
    Some(&line[left..right])
}

/// Returns the hover content for the given token, or `None` if the token does
/// not have hover data.
fn lookup_hover(token: &str) -> Option<&'static str> {
    match token {
        "<PLAYER_SETUP>" => Some("# Player Setup"),
        "<LAND_GENERATION>" => Some("# Land Generation"),
        "<ELEVATION_GENERATION>" => Some("# Elevation Generation"),
        "<CLIFF_GENERATION>" => Some("# Cliff Generation"),
        "<TERRAIN_GENERATION>" => Some("# Terrain Generation"),
        "<CONNECTION_GENERATION>" => Some("# Connection Generation"),
        "<OBJECTS_GENERATION>" => Some("# Objects Generation"),
        _ => None,
    }
}
