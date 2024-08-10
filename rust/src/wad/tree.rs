// use itertools::Itertools as _;
use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{Cursor, Read, Seek},
    path::{self, Path},
};
use thiserror::Error;

use league_toolkit::core::wad::{Wad, WadChunk, WadDecoder, WadError};
use wasm_bindgen::prelude::*;

use crate::{
    league_file::{get_extension_from_league_file_kind, identify_league_file, LeagueFileKind},
    HASHTABLE,
};

use super::Item;

#[derive(Error, Debug)]
pub enum WadTreeError {
    #[error("invalid item name (chunk_path: {chunk_path:#0x})")]
    InvalidItemName { chunk_path: u64 },

    #[error("invalid item path (item_path: {item_path})")]
    InvalidItemPath { item_path: String },

    #[error("failed to create item (item_path: {item_path})")]
    ItemCreationFailure { item_path: String },

    #[error("existing file: (file_path: {file_path})")]
    ExistingFile { file_path: String },

    #[error("parent does not exist: (parent_path: {parent_path})")]
    ParentDoesNotExist { parent_path: String },

    #[error("item does not exist: (item_id: {item_id})")]
    ItemDoesNotExist { item_id: usize },

    #[error("not a directory: (item_id: {item_id})")]
    NotADirectory { item_id: usize },
    #[error("wad error: {0}")]
    WadError(#[from] WadError),
    #[error("{message}")]
    Other { message: String },
}

// type WadTreeError = ();

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct WadItemKindPath {
    // pub kind: ItemKind,
    pub path: String,
}

#[wasm_bindgen]
pub struct WadTree {
    #[wasm_bindgen(getter_with_clone)]
    pub children: Vec<usize>,
    item_storage: Vec<Item>,
    path_lookup: HashMap<String, usize>,
    wad: Option<Wad<Cursor<Vec<u8>>>>,
}

#[wasm_bindgen]
impl WadTree {
    pub(crate) fn new(buf: Vec<u8>) -> Result<Self, WadTreeError> {
        let mut wad = Wad::mount(Cursor::new(buf))?;
        let mut tree = WadTree {
            children: Vec::new(),
            item_storage: Vec::new(),
            path_lookup: HashMap::new(),
            wad: None,
        };

        let (mut decoder, chunks) = wad.decode();

        for (path_hash, chunk) in chunks.iter() {
            let path = unsafe {
                HASHTABLE
                    .as_ref()
                    .and_then(|table| table.try_resolve_path(*path_hash))
                    .map(Ok)
                    .unwrap_or_else(|| Self::guess_chunk_path(*path_hash, chunk, &mut decoder))?
            };
            tree.push_chunk(chunk, path)?;
        }

        for i in 0..tree.item_storage.len() {
            let Some(children) = tree.item_storage.get(i).unwrap().children.clone() else {
                continue;
            };
            let children = children
                .iter()
                .map(|i| (*i, tree.item_storage.get(*i).unwrap()))
                .sorted_by(|(_, a), (_, b)| {
                    let a_dir = a.is_dir();
                    let b_dir = b.is_dir();
                    if a_dir == b_dir {
                        // alphabetical if both are same type
                        return a.name.cmp(&b.name);
                    }
                    // b_dir now implies !a_dir
                    // dirs always come after files
                    match b_dir {
                        true => std::cmp::Ordering::Greater,
                        false => std::cmp::Ordering::Less,
                    }
                })
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();

            tree.item_storage
                .get_mut(i)
                .unwrap()
                .children
                .replace(children);
        }
        tree.wad.replace(wad);
        Ok(tree)
    }

    pub fn get(&self, idx: usize) -> Option<Item> {
        self.item_storage.get(idx).cloned()
    }

    pub fn load_chunk_data(&mut self, idx: usize) -> Result<Box<[u8]>, JsValue> {
        let item = self
            .get(idx)
            .ok_or_else(|| "Item does not exist!".to_string())?;

        let (mut decoder, chunks) = self.wad.as_mut().unwrap().decode();

        let data = decoder
            .load_chunk_decompressed(
                chunks
                    .get(&item.path_hash)
                    .ok_or_else(|| "Could not find Wad chunk for item!".to_string())?,
            )
            .map_err(|e| format!("Could not load chunk - {e:?}"))?;

        Ok(data)
    }

    fn push_chunk(&mut self, chunk: &WadChunk, path: impl AsRef<Path>) -> Result<(), WadTreeError> {
        let components = path.as_ref().components();

        let mut parent: Option<usize> = None;

        let mut cur_path = String::new();
        for component in components {
            let path::Component::Normal(component) = component else {
                return Err(WadTreeError::InvalidItemPath {
                    item_path: path.as_ref().to_string_lossy().into_owned(),
                });
            };

            let component = component.to_string_lossy();
            cur_path += &component;

            if let Some(existing) = self.path_lookup.get(&cur_path) {
                parent = Some(*existing);
                continue;
            }

            let idx = self.item_storage.len();
            self.item_storage.push(Item {
                id: idx,
                name: component.into(),
                children: None,
                path_hash: chunk.path_hash,
                size: chunk.uncompressed_size,
            });

            self.path_lookup.insert(cur_path.clone(), idx);

            match parent {
                Some(parent) => {
                    let parent = self
                        .item_storage
                        .get_mut(parent)
                        .expect("index must always point at real item");
                    match &mut parent.children {
                        Some(children) => children.push(idx),
                        None => parent.children = Some(vec![idx]),
                    }
                }
                None => self.children.push(idx),
            }
            parent = Some(idx);
        }

        Ok(())
    }

    fn guess_chunk_path<R: Read + Seek>(
        path_hash: u64,
        chunk: &WadChunk,
        decoder: &mut WadDecoder<R>,
    ) -> Result<String, WadError> {
        let data = decoder.load_chunk_decompressed(chunk)?;
        let file_kind = identify_league_file(&data);

        match file_kind {
            LeagueFileKind::Unknown => Ok(format!("{:#0x}", path_hash)),
            _ => Ok(format!(
                "{:#0x}.{}",
                path_hash,
                get_extension_from_league_file_kind(file_kind)
            )),
        }
        // Ok("fs".into())
    }
}
