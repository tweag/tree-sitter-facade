mod error;
mod input_edit;
mod language;
mod logger;
mod node;
mod parser;
mod point;
mod query;
mod query_capture;
//mod query_cursor;
mod query_match;
//mod query_matches;
mod range;
mod tree;
mod tree_cursor;

pub use error::*;
pub use input_edit::*;
pub use language::*;
pub use logger::*;
pub use node::*;
pub use parser::*;
pub use point::*;
pub use query::*;
pub use query_capture::*;
//pub use query_cursor::*;
pub use query_match::*;
//pub use query_matches::*;
pub use range::*;
pub use tree::*;
pub use tree_cursor::*;

use std::error::Error;
use std::fmt::Display;

pub struct TreeSitter;

#[derive(Debug)]
struct WasmError {}

impl Display for WasmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for WasmError {}

impl TreeSitter {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn init() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn init() -> Result<(), Box<dyn Error>> {
        match web_tree_sitter::TreeSitter::init().await {
            Err(wasm_bindgen::JsError { .. }) => {
                // TODO: Transfer error message
                Err(Box::new(WasmError {}))
            },
            _ => Ok(()),
        }
    }
}
