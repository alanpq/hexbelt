use std::collections::HashMap;
use std::io::Cursor;

use itertools::Itertools;
use league_toolkit::core::meta::{self, BinTree, ParseError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::{log_object, utils::AsJSError};

#[wasm_bindgen]
pub struct Bin {
    pub version: u32,
    #[wasm_bindgen(getter_with_clone)]
    pub tree: Tree,
}

#[derive(Clone, Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Tree {
    pub objects: Vec<BinEntry>,
}

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
    PropertyJSValue(Value),
    PropertyNone,
    PropertyOptional(Option<Box<BinEntryValue>>),
    PropertyContainer,
    PropertyUnorderedContainer,
    PropertyMap,
    PropertyMapEntry {
        key: Box<BinEntryValue>,
        value: Box<BinEntryValue>,
    },
    PropertyStruct {
        class: String,
    },
    PropertyEmbedded {
        class: String,
    },
}

impl BinEntryValue {
    pub fn from_prop_value(
        value: &meta::property::value::PropertyValueEnum,
    ) -> (Self, Option<Vec<BinEntry>>) {
        match value {
            meta::property::value::PropertyValueEnum::None(_) => {
                (BinEntryValue::PropertyNone, None)
            }
            meta::property::value::PropertyValueEnum::Container(v) => (
                BinEntryValue::PropertyContainer,
                Some(
                    v.items
                        .iter()
                        .enumerate()
                        .map(|(idx, i)| BinEntry::from_value(Some(idx.to_string()), i))
                        .collect(),
                ),
            ),
            meta::property::value::PropertyValueEnum::UnorderedContainer(v) => (
                BinEntryValue::PropertyUnorderedContainer,
                Some(
                    v.0.items
                        .iter()
                        .enumerate()
                        .map(|(idx, i)| BinEntry::from_value(Some(idx.to_string()), i))
                        .collect(),
                ),
            ),
            meta::property::value::PropertyValueEnum::Map(v) => (
                BinEntryValue::PropertyMap,
                Some(
                    v.entries
                        .iter()
                        .map(|(k, v)| BinEntry {
                            name: None,
                            value: BinEntryValue::PropertyMapEntry {
                                key: Box::new(BinEntryValue::from_prop_value(&k.0).0),
                                value: Box::new(BinEntryValue::from_prop_value(v).0),
                            },
                            children: vec![],
                        })
                        .collect(),
                ),
            ),
            meta::property::value::PropertyValueEnum::Struct(v) => (
                BinEntryValue::PropertyStruct {
                    class: v.class_hash.to_string(),
                },
                Some(v.properties.values().map_into().collect()),
            ),
            meta::property::value::PropertyValueEnum::Embedded(v) => (
                BinEntryValue::PropertyEmbedded {
                    class: v.0.class_hash.to_string(),
                },
                Some(v.0.properties.values().map_into().collect()),
            ),

            meta::property::value::PropertyValueEnum::Optional(v) => match &v.0 {
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
                tracing::debug!("raw js: {value:?}");
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
    pub fn from_value(
        name: Option<String>,
        value: &meta::property::value::PropertyValueEnum,
    ) -> Self {
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
            name: Some(format!("{:#x}", prop.name_hash)),
            value,
            children: children.unwrap_or_default(),
        }
    }
}
impl From<meta::BinTreeObject> for BinEntry {
    fn from(obj: meta::BinTreeObject) -> Self {
        Self {
            name: Some(format!("{:#x}", obj.path_hash)),
            value: BinEntryValue::Object,
            children: obj.properties.values().map_into().collect(),
        }
    }
}

#[wasm_bindgen]
impl Bin {
    #[wasm_bindgen(js_name = "from_bytes")]
    pub fn from_bytes_js(bytes: Box<[u8]>) -> Result<Bin, JsValue> {
        Self::from_bytes(&bytes).map_err(AsJSError::into_js_error)
    }
}

use gloo_utils::format::JsValueSerdeExt;
impl Bin {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        tracing::debug!("reading bin file ({} bytes)...", bytes.len());
        let tree = BinTree::from_reader(&mut Cursor::new(bytes))?;

        Ok(Bin {
            version: tree.version,
            tree: Tree {
                objects: tree.objects.into_values().map_into().collect(),
            },
        })
    }
}
