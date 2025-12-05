use std::collections::HashMap;
use std::io::Cursor;

use itertools::Itertools;
use league_toolkit::meta::{self, value::PropertyValueEnum, BinTree};
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
    pub path_hash: u32,
    pub class_hash: u32,
    pub properties: Vec<BinProperty>,
}

impl From<meta::BinTreeObject> for BinObject {
    fn from(value: meta::BinTreeObject) -> Self {
        Self {
            name: value.path_hash.to_string(),
            path_hash: value.path_hash,
            class_hash: value.class_hash,
            properties: value.properties.into_values().map_into().collect(),
        }
    }
}

#[derive(Clone, Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BinProperty {
    pub name: String,
    pub value: Value,
}

impl From<meta::BinProperty> for BinProperty {
    fn from(value: meta::BinProperty) -> Self {
        Self {
            name: value.name_hash.to_string(),
            value: serde_json::to_value(value.value).unwrap(),
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

impl BinEntryValue {
    pub fn from_prop_value(value: &PropertyValueEnum) -> (Self, Option<Vec<BinEntry>>) {
        match value {
            PropertyValueEnum::None(_) => (BinEntryValue::PropertyNone, None),
            PropertyValueEnum::Container(v) => (
                BinEntryValue::PropertyContainer,
                Some(
                    v.items
                        .iter()
                        .enumerate()
                        .map(|(idx, i)| BinEntry::from_value(Some(idx.to_string()), i))
                        .collect(),
                ),
            ),
            PropertyValueEnum::UnorderedContainer(v) => (
                BinEntryValue::PropertyUnorderedContainer,
                Some(
                    v.0.items
                        .iter()
                        .enumerate()
                        .map(|(idx, i)| BinEntry::from_value(Some(idx.to_string()), i))
                        .collect(),
                ),
            ),
            PropertyValueEnum::Map(v) => (
                BinEntryValue::PropertyMap,
                Some(
                    v.entries
                        .iter()
                        .map(|(k, v)| {
                            let value = BinEntryValue::from_prop_value(v);
                            BinEntry {
                                name: None,
                                value: BinEntryValue::PropertyMapEntry {
                                    key: Box::new(BinEntryValue::from_prop_value(&k.0).0),
                                    value: Box::new(value.0),
                                },
                                children: value.1.unwrap_or_default(),
                            }
                        })
                        .collect(),
                ),
            ),
            PropertyValueEnum::Struct(v) => (
                BinEntryValue::PropertyStruct {
                    class_name: unsafe { BIN_TYPES.as_ref() }
                        .and_then(|types| types.try_resolve_path(v.class_hash)),

                    class: v.class_hash.to_string(),
                },
                Some(v.properties.values().map_into().collect()),
            ),
            PropertyValueEnum::Embedded(v) => (
                BinEntryValue::PropertyEmbedded {
                    class_name: unsafe { BIN_TYPES.as_ref() }
                        .and_then(|types| types.try_resolve_path(v.0.class_hash)),
                    class: v.0.class_hash.to_string(),
                },
                Some(v.0.properties.values().map_into().collect()),
            ),

            PropertyValueEnum::Optional(v) => match &v.value {
                Some(inner) => {
                    let (inner, children) = BinEntryValue::from_prop_value(inner);
                    (
                        BinEntryValue::PropertyOptional(Some(Box::new(inner))),
                        children,
                    )
                }
                None => (BinEntryValue::PropertyOptional(None), None),
            },
            _ => {
                // tracing::debug!("raw js: {value:?}");
                (
                    BinEntryValue::PropertyJSValue(serde_json::to_value(value).unwrap()),
                    None,
                )
            }
        }
    }
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
