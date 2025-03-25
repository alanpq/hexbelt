pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

use wasm_bindgen::prelude::*;
pub trait AsJSError {
    fn into_js_error(self) -> JsValue
    where
        Self: std::marker::Sized,
    {
        self.as_js_error()
    }
    fn as_js_error(&self) -> JsValue;
}

impl AsJSError for league_toolkit::core::meta::Error {
    fn as_js_error(&self) -> JsValue {
        format!("Failed to parse bin: {self:?}").into()
    }
}

impl AsJSError for crate::WadTreeError {
    fn as_js_error(&self) -> JsValue {
        format!("Failed to parse/build Wad tree: {self:?}").into()
    }
}
