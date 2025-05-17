use std::collections::HashMap;
use std::io::Cursor;

use itertools::Itertools;
use league_toolkit::core::meta::{self, value::PropertyValueEnum, BinTree};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use super::BinEntry;

#[derive(Clone, Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "kind", content = "value")]
pub enum TreeNode {
    Namespace(String, HashMap<String, TreeNode>),
    Object(String, u32),
}

impl TreeNode {
    pub(super) fn insert_object(&mut self, obj: &BinEntry, obj_hash: u32) {
        let Some(name) = &obj.name else {
            return;
        };
        tracing::debug!("PATH = {name}");
        let components = name.split('/').collect_vec();
        self.insert(&components, obj_hash);
    }

    fn insert(&mut self, path: &[&str], obj: u32) {
        match path.split_first() {
            Some((first, rest)) => match self {
                Self::Namespace(_, children) => {
                    let first = first.to_string();
                    let child = children
                        .entry(first.clone())
                        .or_insert_with(|| match rest.len() {
                            0 => Self::Object(first, obj),
                            _ => Self::Namespace(first, HashMap::new()),
                        });
                    child.insert(rest, obj);
                }
                Self::Object(name, id) => {
                    tracing::warn!("Cannot insert into a leaf node path: \n\npath = {path:?}\n\nself = {self:?}\n\nobj = {obj:?}");
                }
            },
            None => {
                tracing::debug!("called insert with no path left");
            }
        }
    }
}
