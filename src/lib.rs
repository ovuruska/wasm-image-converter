use wasm_bindgen::prelude::*;
use image::{load_from_memory, ImageOutputFormat};
use std::io::Cursor;

#[wasm_bindgen]
pub fn convert_image(input: &[u8], format: &str) -> Vec<u8> {
    // Görseli belleğe yükle
    let img = load_from_memory(input).expect("Görüntü okunamadı");

    let mut buf: Vec<u8> = Vec::new();

    // Formatı belirle
    let fmt = match format.to_lowercase().as_str() {
        "png" => ImageOutputFormat::Png,
        "jpeg" | "jpg" => ImageOutputFormat::Jpeg(80),
        "webp" => ImageOutputFormat::WebP,
        "bmp" => ImageOutputFormat::Bmp,
        "gif" => ImageOutputFormat::Gif,
        "tiff" => ImageOutputFormat::Tiff,
        _ => panic!("Desteklenmeyen format: {}", format),
    };

    // Görseli istenen formata dönüştür
    img.write_to(&mut Cursor::new(&mut buf), fmt)
        .expect("Görsel dönüştürülemedi");

    buf
}