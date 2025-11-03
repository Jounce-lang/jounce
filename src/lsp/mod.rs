// LSP Server Module
// Implements Language Server Protocol for Jounce
// Session 28 - Full LSP Implementation

pub mod server;
pub mod backend;
pub mod capabilities;
pub mod completion;
pub mod lsp_diagnostics;
pub mod hover;
pub mod goto_definition;

pub use server::run_lsp_server;

// Re-export LSP types for use in other modules (e.g., diagnostics.rs)
pub use lsp_types::{Range, Position};
