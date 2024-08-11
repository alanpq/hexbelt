use std::io::Cursor;

use league_toolkit::core::meta::{BinTree, ParseError};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Bin {
    tree: BinTree,
}

#[wasm_bindgen]
impl Bin {
    #[wasm_bindgen(js_name = "from_bytes")]
    pub fn from_bytes_js(bytes: Box<[u8]>) -> Result<Bin, JsValue> {
        Self::from_bytes(&bytes).map_err(|e| format!("failed to read bin from bytes: {e:?}").into())
    }
}

impl Bin {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        tracing::debug!("reading bin file ({} bytes)...", bytes.len());
        Ok(Bin {
            tree: BinTree::from_reader(&mut Cursor::new(bytes))?,
        })
    }
}
