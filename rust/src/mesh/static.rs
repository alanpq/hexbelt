#[wasm_bindgen(js_name = "open_wad")]
pub async fn load_static_mesh(file: File) -> Result<WadTree, JsValue> {
    WadTree::new(read_file(file).await?).map_err(AsJSError::into_js_error)
}
