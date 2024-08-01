pub mod league_file;
pub mod utils;
pub mod wad;

use utils::set_panic_hook;
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

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! log{
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: String);
}

#[wasm_bindgen(js_name = "open_wad")]
pub async fn open_wad(file: File) -> Result<WadTree, JsValue> {
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
    log!("file opened!");
    WadTree::new(vec).map_err(|e| format!("error: {e:?}").into())
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    tracing_wasm::set_as_global_default();

    Ok(())
}
