use std::io::Cursor;

use league_toolkit::core::render::texture::CompressedTexture;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    #[wasm_bindgen(getter_with_clone)]
    pub data: Vec<u8>,
}
#[wasm_bindgen(js_name = "decode_texture")]
pub fn decode_texture(data: Box<[u8]>) -> Result<Texture, JsValue> {
    let tex = CompressedTexture::from_reader(&mut Cursor::new(data)).map_err(|e| e.to_string())?;
    let (width, height) = (tex.width(), tex.height());
    let img = tex.to_rgba_image(0).map_err(|e| e.to_string())?;

    Ok(Texture {
        width,
        height,
        data: img.into_raw(),
    })
}
