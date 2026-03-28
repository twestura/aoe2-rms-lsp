//! Aoe2 RMS Language Server

mod instructions; // TODO remove/refactor to parser
mod parser;

use parser::RmsDocument;

use std::collections::HashMap;

use tokio::sync::RwLock;
use tower_lsp::Client;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CompletionOptions, CompletionParams, CompletionResponse, DidChangeTextDocumentParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, Hover, HoverParams,
    HoverProviderCapability, PositionEncodingKind, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, Url,
};
use tower_lsp::{
    LanguageServer, LspService, Server,
    lsp_types::{InitializeParams, InitializeResult, InitializedParams, MessageType},
};

/// The server's in-memory state.
#[derive(Debug)]
struct Backend {
    /// Handle for communicating with the client.
    client: Client,
    /// The server's in-memory document store.
    /// Maps a document's URI to its contents.
    documents: RwLock<HashMap<Url, RmsDocument>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions::default()),
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
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let rms_doc = RmsDocument::parse(text);
        self.documents.write().await.insert(uri, rms_doc);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        // For now we use the full text document sync kind that returns
        // the entire document in the change. This is fine for small map
        // scripts (many maps are just a few hundred lines) during development.
        let text = params.content_changes.into_iter().next().unwrap().text;
        let rms_doc = RmsDocument::parse(text);
        self.documents.write().await.insert(uri, rms_doc);
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.write().await.remove(&uri);
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let documents = self.documents.read().await;
        Ok(documents.get(&uri).and_then(|rms| rms.hover(position)))
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let documents = self.documents.read().await;
        Ok(documents.get(&uri).and_then(|rms| rms.completion(position)))
    }
}

#[tokio::main]
async fn main() {
    // std::panic::set_hook(Box::new(|info| {
    //     _log(&format!("PANIC: {info}"));
    // }));
    // _log(&format!(
    //     "TERRAIN_CONSTANTS count: {}",
    //     predefined::TERRAIN_CONSTANTS.len()
    // ));
    // _log(&format!(
    //     "GAME_MODE_LABELS count: {}",
    //     predefined::GAME_MODE_LABELS.len()
    // ));
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: RwLock::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}

/// Debugging function for logging messages from the LSP to a file.
fn _log(msg: &str) {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("aoe2-rms-lsp.txt")
        .unwrap();
    writeln!(file, "{msg}").unwrap();
}
