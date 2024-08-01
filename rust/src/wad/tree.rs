// use itertools::Itertools as _;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Cursor, Read, Seek},
    path::{self, Path},
};
// use thiserror::Error;

use league_toolkit::core::wad::{Wad, WadChunk, WadDecoder, WadError};
use wasm_bindgen::prelude::*;

use crate::league_file::{
    get_extension_from_league_file_kind, identify_league_file, LeagueFileKind,
};

use super::{Directory, File, Item, ItemKind};

// #[derive(Error, Debug)]
// pub enum WadTreeError {
//     #[error("invalid item name (chunk_path: {chunk_path:#0x})")]
//     InvalidItemName { chunk_path: u64 },

//     #[error("invalid item path (item_path: {item_path})")]
//     InvalidItemPath { item_path: String },

//     #[error("failed to create item (item_path: {item_path})")]
//     ItemCreationFailure { item_path: String },

//     #[error("existing file: (file_path: {file_path})")]
//     ExistingFile { file_path: String },

//     #[error("parent does not exist: (parent_path: {parent_path})")]
//     ParentDoesNotExist { parent_path: String },

//     #[error("item does not exist: (item_id: {item_id})")]
//     ItemDoesNotExist { item_id: usize },

//     #[error("not a directory: (item_id: {item_id})")]
//     NotADirectory { item_id: usize },

//     #[error("wad error: {0}")]
//     WadError(#[from] WadError),

//     #[error("{message}")]
//     Other { message: String },
// }

type WadTreeError = ();

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct WadItemKindPath {
    pub kind: ItemKind,
    pub path: String,
}

#[wasm_bindgen]
pub struct WadTree {
    items: Vec<usize>,
    item_storage: Vec<Item>,
    chunk_item_ids: HashMap<WadItemKindPath, usize>,
}

#[wasm_bindgen]
pub struct Children {
    dirs: Vec<Directory>,
    files: Vec<File>,
}

#[wasm_bindgen]
impl WadTree {
    pub fn children(&self) -> Children {
        Children {
            dirs: self
                .items
                .iter()
                .filter_map(|id| match self.item_storage.get(*id).cloned()? {
                    Item::File(_) => None,
                    Item::Directory(f) => Some(f),
                })
                .collect(),
            files: self
                .items
                .iter()
                .filter_map(|id| match self.item_storage.get(*id).cloned()? {
                    Item::File(f) => Some(f),
                    Item::Directory(_) => None,
                })
                .collect(),
        }
    }

    pub(crate) fn new(buf: Vec<u8>) -> Result<Self, WadError> {
        let mut wad = Wad::mount(Cursor::new(buf))?;
        let mut tree = WadTree {
            items: Vec::new(),
            item_storage: Vec::new(),
            chunk_item_ids: HashMap::new(),
        };

        let (mut decoder, chunks) = wad.decode();

        for (path_hash, chunk) in chunks.iter() {
            let path = Self::guess_chunk_path(*path_hash, chunk, &mut decoder);
        }

        Ok(tree)
    }

    fn push_chunk(&mut self, chunk: &WadChunk, path: impl AsRef<Path>) -> Result<(), WadTreeError> {
        let mut components = path.as_ref().components().peekable();

        let parent: Option<&mut Directory> = None;

        // let parent_id: Option<usize> = None;
        // let parent_path = Some(PathBuf::new());
        // while let Some(component) = components.next() {
        //     let path::Component::Normal(component) = component else {
        //         return Err(WadTreeError::InvalidItemPath {
        //             item_path: path.as_ref().to_string_lossy().into_owned(),
        //         });
        //     };

        //     let cur_path = match &parent_path {
        //         Some(cur_path) => cur_path.join(component),
        //         None => todo!(),
        //     };
        // }

        // self.item_storage.push(Item)
        let Some(path::Component::Normal(last)) = components.last() else {
            // return Err(WadTreeError::InvalidItemPath {
            //     item_path: path.as_ref().to_string_lossy().into_owned(),
            // });
            return Err(());
        };

        let idx = self.item_storage.len();
        self.item_storage.push(Item::File(File {
            name: last.to_string_lossy().to_string(),
        }));
        self.items.push(idx);
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
