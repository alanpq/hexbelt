pub mod hash_table;
pub mod league_file;
pub mod meta;
pub mod tex;
pub mod utils;
pub mod wad;

use hash_table::HashTable;
pub use meta::Bin;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt as _, Registry};
use tracing_wasm::{WASMLayer, WASMLayerConfigBuilder};
use utils::{set_panic_hook, AsJSError};
use wad::*;
use wasm_bindgen::prelude::*;
use web_sys::File;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_object(a: JsValue);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u8array(a: js_sys::Uint8Array);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[macro_export]
macro_rules! log{
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: String);
}

static mut WAD_HASHTABLE: Option<HashTable> = None;
static mut BIN_FIELDS: Option<HashTable> = None;
static mut BIN_PATHS: Option<HashTable> = None;
static mut BIN_TYPES: Option<HashTable<u32>> = None;

async fn read_file(file: File) -> Result<Vec<u8>, JsValue> {
    let promise = js_sys::Promise::new(&mut move |res, _rej| {
        let file_reader = web_sys::FileReader::new().unwrap();

        let fr = file_reader.clone();
        let closure = Closure::wrap(Box::new(move || {
            res.call1(&JsValue::undefined(), &fr.result().unwrap())
                .unwrap();
        }) as Box<dyn FnMut()>);

        file_reader.set_onload(Some(closure.as_ref().unchecked_ref()));

        closure.forget();

        file_reader.read_as_array_buffer(&file).unwrap();
    });
    let fut = wasm_bindgen_futures::JsFuture::from(promise);
    log!("opening wad...");
    let res = fut.await?;
    let buffer = js_sys::Uint8Array::new(&res);
    let mut vec = vec![0; buffer.length() as _];
    buffer.copy_to(&mut vec[..]);
    Ok(vec)
}

#[wasm_bindgen(js_name = "open_wad")]
pub async fn open_wad(file: File) -> Result<WadTree, JsValue> {
    WadTree::new(read_file(file).await?).map_err(AsJSError::into_js_error)
}

#[wasm_bindgen(js_name = "open_bin")]
pub async fn open_bin(file: File) -> Result<Bin, JsValue> {
    Bin::from_bytes(&read_file(file).await?).map_err(AsJSError::into_js_error)
}

#[wasm_bindgen(js_name = "load_wad_hashtables")]
pub async fn load_wad_hashtables(base: String) -> Result<usize, JsValue> {
    let mut table = HashTable::new();

    const FILES: [&str; 3] = ["hashes.game.txt.0", "hashes.game.txt.1", "hashes.lcu.txt"];
    let count = table.load(base, FILES).await?;
    unsafe { WAD_HASHTABLE.replace(table) };
    Ok(count)
}
#[wasm_bindgen(js_name = "load_bin_hashtables")]
pub async fn load_bin_hashtables(base: String) -> Result<usize, JsValue> {
    let mut count = 0;

    let mut table = HashTable::new();
    count += table.load(&base, ["hashes.binfields.txt"]).await?;
    unsafe { BIN_FIELDS.replace(table) };

    let mut table = HashTable::new();
    count += table.load(&base, ["hashes.binentries.txt"]).await?;
    unsafe { BIN_PATHS.replace(table) };

    let mut table = HashTable::new();
    count += table.load(&base, ["hashes.bintypes.txt"]).await?;
    unsafe { BIN_TYPES.replace(table) };

    Ok(count)
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();

    LogTracer::init().unwrap();
    let layer_config = WASMLayerConfigBuilder::new()
        .set_max_level(tracing::Level::TRACE)
        .build();
    tracing::subscriber::set_global_default(Registry::default().with(WASMLayer::new(layer_config)))
        .expect("default global");

    Ok(())
}
