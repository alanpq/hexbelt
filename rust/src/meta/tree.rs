use std::collections::HashMap;
use std::io::Cursor;

use itertools::Itertools;
use league_toolkit::core::meta::{self, value::PropertyValueEnum, BinTree};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::utils::AsJSError;

use super::{BinEntry, BinEntryValue, TreeNode};

#[wasm_bindgen]
pub struct Bin {
    pub version: u32,
    #[wasm_bindgen(getter_with_clone)]
    pub data: Data,
}

#[derive(Clone, Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Data {
    pub tree: TreeNode,
    pub objects: HashMap<u32, BinEntry>,
}

#[derive(Clone, Debug, Tsify, Serialize, Deserialize)]
pub struct TreeEdits {
    value_edits: HashMap<u32, Vec<ObjectValueEdit>>,
}

#[derive(Clone, Debug, Tsify, Serialize, Deserialize)]
pub struct ObjectValueEdit {
    pub path: Vec<u32>,
    pub value: BinEntryValue,
}

#[wasm_bindgen]
impl Bin {
    #[wasm_bindgen(js_name = "from_bytes")]
    pub fn from_bytes_js(bytes: Box<[u8]>) -> Result<Bin, JsValue> {
        Self::from_bytes(&bytes).map_err(AsJSError::into_js_error)
    }
}
impl Bin {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, meta::Error> {
        // tracing::debug!("reading bin file ({} bytes)...", bytes.len());
        let bin = BinTree::from_reader(&mut Cursor::new(bytes))?;

        let objects = bin
            .objects
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect::<HashMap<_, _>>();

        let mut root = TreeNode::Namespace("<root>".into(), HashMap::new());
        for (hash, obj) in objects.iter() {
            root.insert_object(obj, *hash);
        }

        Ok(Bin {
            version: bin.version,

            data: Data {
                tree: root,
                objects: objects
                    .into_iter()
                    .map(|(k, mut v)| {
                        if let Some(name) = v.name.as_ref().and_then(|n| {
                            n.rsplit_once('/').map(|(l, r)| match r.is_empty() {
                                true => l.to_string(),
                                false => r.to_string(),
                            })
                        }) {
                            v.name.replace(name);
                        }
                        (k, v)
                    })
                    .collect(),
            },
        })
    }
}
