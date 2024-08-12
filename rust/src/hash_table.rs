use std::{collections::HashMap, sync::Mutex};
use tracing::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
// use walkdir::WalkDir;

#[derive(Debug, Clone, Default)]
pub struct HashTable {
    is_loaded: bool,
    items: HashMap<u64, String>,
}

impl HashTable {
    pub fn new() -> Self {
        HashTable {
            is_loaded: false,
            items: HashMap::default(),
        }
    }

    pub fn try_resolve_path(&self, path_hash: u64) -> Option<String> {
        self.items.get(&path_hash).cloned()
    }

    pub async fn load<S: AsRef<str>, const N: usize>(
        &mut self,
        base: S,
        files: [&'static str; N],
    ) -> Result<usize, JsValue> {
        let base = base.as_ref();
        // const FILES: [&str; 3] = ["hashes.game.txt.0", "hashes.game.txt.1", "hashes.lcu.txt"];

        info!("loading hash files...");

        for file in files {
            let _ = self.load_from_url(format!("{base}/{file}")).await;
        }
        info!("loaded hash files! {} entries", self.items.len());

        self.is_loaded = true;

        Ok(self.items.len())
    }

    pub async fn load_from_url(&mut self, url: String) -> Result<(), JsValue> {
        // let reader = BufReader::new(file);
        use web_sys::{Request, RequestInit, Response};
        let mut opts = RequestInit::new();
        opts.method("GET");
        // opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&url, &opts)?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

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
}

pub struct WadHashtableState(pub Mutex<HashTable>);
