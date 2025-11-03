// LSP Backend - Main Language Server Implementation
// Session 28

use dashmap::DashMap;
use lsp_types::*;
use tower_lsp::jsonrpc::Result;
use tower_lsp::{Client, LanguageServer};
use std::sync::Arc;

use super::capabilities::server_capabilities;
use super::completion::get_completions;
use super::lsp_diagnostics::analyze_document;
use super::hover::get_hover_info;
use super::goto_definition::find_definition;

pub struct JounceLanguageServer {
    client: Client,
    documents: Arc<DashMap<String, String>>,
}

impl JounceLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(DashMap::new()),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for JounceLanguageServer {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "Jounce Language Server".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: server_capabilities(),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Jounce LSP server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;
        
        self.documents.insert(uri.clone(), text.clone());
        
        // Run diagnostics
        let diagnostics = analyze_document(&text);
        self.client
            .publish_diagnostics(params.text_document.uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        
        if let Some(change) = params.content_changes.first() {
            self.documents.insert(uri.clone(), change.text.clone());
            
            // Run diagnostics
            let diagnostics = analyze_document(&change.text);
            self.client
                .publish_diagnostics(params.text_document.uri, diagnostics, None)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.documents.remove(&uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        
        if let Some(doc) = self.documents.get(&uri) {
            let position = params.text_document_position.position;
            let completions = get_completions(&doc, position);
            Ok(Some(CompletionResponse::Array(completions)))
        } else {
            Ok(None)
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        
        if let Some(doc) = self.documents.get(&uri) {
            let position = params.text_document_position_params.position;
            Ok(get_hover_info(&doc, position))
        } else {
            Ok(None)
        }
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        
        if let Some(doc) = self.documents.get(&uri) {
            let position = params.text_document_position_params.position;
            let location = find_definition(&doc, position, &params.text_document_position_params.text_document.uri);
            Ok(location.map(GotoDefinitionResponse::Scalar))
        } else {
            Ok(None)
        }
    }
}
