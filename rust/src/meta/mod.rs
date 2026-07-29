use std::collections::HashMap;
use std::io::Cursor;

use itertools::Itertools;
use league_toolkit::{
    hash::BinHash,
    meta::{self, BinTree, PropertyValueEnum},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::{log_object, utils::AsJSError, BIN_FIELDS, BIN_PATHS, BIN_TYPES};

mod node;
mod tree;

pub use node::*;
pub use tree::*;

#[derive(Clone, Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BinObject {
    pub name: String,
    pub path_hash: BinHash,
    pub class_hash: BinHash,
    pub properties: Vec<BinProperty>,
}

impl From<meta::BinObject> for BinObject {
    fn from(value: meta::BinObject) -> Self {
        Self {
            name: value.path_hash.to_string(),
            path_hash: value.path_hash,
            class_hash: value.class_hash,
            properties: value.properties.into_iter().map_into().collect(),
        }
    }
}

#[derive(Clone, Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BinProperty {
    pub name: String,
    pub value: Value,
}

impl From<(BinHash, PropertyValueEnum)> for BinProperty {
    fn from((name, value): (BinHash, PropertyValueEnum)) -> Self {
        Self {
            name: name.to_string(),
            value: serde_json::to_value(value).unwrap(),
        }
    }
}

#[derive(Clone, Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "kind", content = "value")]
pub enum BinEntryValue {
    Object,
    Namespace,
    PropertyJSValue(Value),
    PropertyNone,
    PropertyOptional(Option<Box<BinEntryValue>>),
    PropertyContainer,
    PropertyUnorderedContainer,
    PropertyMap,
    #[serde(rename_all = "camelCase")]
    PropertyMapEntry {
        key: Box<BinEntryValue>,
        value: Box<BinEntryValue>,
    },
    #[serde(rename_all = "camelCase")]
    PropertyStruct {
        class_name: Option<String>,
        class: String,
    },
    #[serde(rename_all = "camelCase")]
    PropertyEmbedded {
        class_name: Option<String>,
        class: String,
    },
}

#[derive(Clone, Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BinEntry {
    pub name: Option<String>,
    pub value: BinEntryValue,
    pub children: Vec<BinEntry>,
}

impl BinEntry {
    pub fn from_value(name: Option<String>, value: &PropertyValueEnum) -> Self {
        let (value, children) = BinEntryValue::from_prop_value(value);
        Self {
            name,
            value,
            children: children.unwrap_or_default(),
        }
    }
}

impl From<&meta::BinProperty> for BinEntry {
    fn from(prop: &meta::BinProperty) -> Self {
        let (value, children) = BinEntryValue::from_prop_value(&prop.value);
        Self {
            name: Some(
                unsafe { BIN_FIELDS.as_ref() }
                    .and_then(|t| t.try_resolve_path(prop.name_hash as _))
                    .unwrap_or_else(|| format!("{:#x}", prop.name_hash)),
            ),
            value,
            children: children.unwrap_or_default(),
        }
    }
}
impl From<meta::BinTreeObject> for BinEntry {
    fn from(obj: meta::BinTreeObject) -> Self {
        Self {
            // name: Some(format!("{:#x}", obj.path_hash)),
            name: Some(
                unsafe { BIN_PATHS.as_ref() }
                    .and_then(|t| t.try_resolve_path(obj.path_hash as _))
                    .unwrap_or_else(|| format!("{:#x}", obj.path_hash)),
            ),
            value: BinEntryValue::Object,
            children: obj.properties.values().map_into().collect(),
        }
    }
}
