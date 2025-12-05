use std::io::Cursor;

use league_toolkit::texture::Texture as LolTexture;
use wasm_bindgen::prelude::*;

use crate::log;

#[wasm_bindgen]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub mipmaps: u32,
    #[wasm_bindgen(getter_with_clone)]
    pub data: Vec<u8>,
}
#[wasm_bindgen(js_name = "decode_texture")]
pub fn decode_texture(data: Box<[u8]>, mipmap: u32) -> Result<Texture, JsValue> {
    let tex = LolTexture::from_reader(&mut Cursor::new(data)).map_err(|e| e.to_string())?;
    if let LolTexture::Tex(tex) = &tex {
        log!("fmt: {:?}", tex.format);
        log!("mips: {}", tex.mip_count);
    }
    let mipmaps = tex.mip_count();
    let img = tex
        .decode_mipmap(mipmap)
        .map_err(|e| e.to_string())?
        .into_rgba_image()
        .map_err(|e| e.to_string())?;
    let (width, height) = (img.width(), img.height());

    Ok(Texture {
        width,
        height,
        mipmaps,
        data: img.into_raw(),
    })
}
