use anyhow::anyhow;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    sync::{Arc, Mutex},
};
use tracing::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
// use walkdir::WalkDir;

#[derive(Debug, Clone, Default)]
pub struct WadHashtable {
    is_loaded: bool,
    items: HashMap<u64, String>,
}

impl WadHashtable {
    pub fn new() -> Self {
        WadHashtable {
            is_loaded: false,
            items: HashMap::default(),
        }
    }

    pub fn try_resolve_path(&self, path_hash: u64) -> Option<String> {
        self.items.get(&path_hash).cloned()
    }

    pub async fn add_from_gh(&mut self) -> Result<(), JsValue> {
        const URL: &str = "/hashes/";
        const FILES: [&str; 3] = ["hashes.game.txt.0", "hashes.game.txt.1", "hashes.lcu.txt"];

        info!("loading hash files...");

        for file in FILES {
            let _ = self.add_from_url(format!("{URL}{file}")).await;
        }
        info!("loaded hash files! {} entries", self.items.len());

        // info!("loading wad hasthables from dir: {:?}", dir.as_ref());

        // for wad_hashtable_entry in WalkDir::new(dir).into_iter().filter_map(|x| x.ok()) {
        //     if !wad_hashtable_entry.file_type().is_file() {
        //         continue;
        //     }

        //     info!("loading wad hasthable: {:?}", wad_hashtable_entry.path());
        //     self.add_from_file(&mut File::open(wad_hashtable_entry.path())?)?;
        // }

        self.is_loaded = true;

        Ok(())
    }

    pub async fn add_from_url(&mut self, url: String) -> Result<(), JsValue> {
        // let reader = BufReader::new(file);
        use web_sys::{Request, RequestInit, RequestMode, Response};
        let mut opts = RequestInit::new();
        opts.method("GET");
        // opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&url, &opts)?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        // `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        // Convert this other `Promise` into a rust `Future`.
        let text = JsFuture::from(resp.text()?).await?;
        let text = text.as_string().expect("response.text() must be string");

        let lines = text.lines();

        for line in lines {
            let mut components = line.split(' ');

            let hash = components
                .next()
                .ok_or(JsValue::from_str("failed to read hash"))?;
            let hash = u64::from_str_radix(hash, 16)
                .map_err(|e| JsValue::from_str(&format!("failed to convert hash - {e:?}")))?;
            let path = itertools::join(components, " ");

            self.items.insert(hash, path);
        }

        Ok(())
    }

    pub(crate) fn items(&self) -> &HashMap<u64, String> {
        &self.items
    }
    pub(crate) fn items_mut(&mut self) -> &mut HashMap<u64, String> {
        &mut self.items
    }
}

pub struct WadHashtableState(pub Mutex<WadHashtable>);
