use std::io::Cursor;

use base64::prelude::*;
use image::{imageops::FilterType::Nearest, load_from_memory};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn edit_image(base_img: &str, select: &str) -> String {
    //base64 to image
    let img_buffer = BASE64_STANDARD.decode(base_img).unwrap();
    let mut img = load_from_memory(img_buffer.as_slice()).unwrap();

    img = match select {
        "half" => img.resize(img.width() / 2, img.height() / 2, Nearest),
        "gray" => img.grayscale(),
        "r90" => img.rotate90(),
        "r180" => img.rotate180(),
        _ => img,
    };

    //image to base64
    let mut buf: Vec<u8> = Vec::new();
    let _ = img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png);
    BASE64_STANDARD.encode(&buf)
}
