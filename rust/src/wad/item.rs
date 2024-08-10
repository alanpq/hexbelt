#![allow(non_snake_case)]
use wasm_bindgen::prelude::*;

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
