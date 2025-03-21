use std::io::Cursor;

use base64::prelude::*;
use image::load_from_memory;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn edit_image(base_img: &str) -> String {
    //base64 to image
    let img_buffer = BASE64_STANDARD.decode(base_img).unwrap();
    let mut img = load_from_memory(img_buffer.as_slice()).unwrap();

    //grayscale
    img = img.grayscale();

    //image to base64
    let mut buf: Vec<u8> = Vec::new();
    let _ = img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png);
    BASE64_STANDARD.encode(&buf)
}
