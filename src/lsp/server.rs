// LSP Server Entry Point
// Session 28 - Tower-LSP Implementation

use tower_lsp::{LspService, Server};

use super::backend::JounceLanguageServer;

pub async fn run_lsp_server() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| {
        JounceLanguageServer::new(client)
    });

    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
