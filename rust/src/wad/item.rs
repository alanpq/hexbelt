#![allow(non_snake_case)]

use std::path::PathBuf;
use std::sync::Arc;
use std::vec;
use std::{collections::HashMap, path::Path};

use js_sys::JsString;
use league_toolkit::core::wad::WadChunk;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;
use xxhash_rust::xxh3::xxh3_64;

use super::WadTree;

#[derive(Debug, Clone, PartialEq, Eq)]
#[wasm_bindgen]
pub struct Item {
    pub id: usize,
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
    #[wasm_bindgen(getter_with_clone)]
    pub children: Option<Vec<usize>>,
    pub size: usize,

    pub(crate) path_hash: u64,
}

#[wasm_bindgen]
impl Item {
    pub fn is_file(&self) -> bool {
        self.children.is_none()
    }
    pub fn is_dir(&self) -> bool {
        self.children.is_some()
    }
}
