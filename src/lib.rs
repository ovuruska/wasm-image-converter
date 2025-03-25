use wasm_bindgen::prelude::*;
use image::{load_from_memory, ImageOutputFormat, DynamicImage};
use std::io::Cursor;
use web_sys::console;

#[wasm_bindgen]
pub fn convert_image(input: &[u8], format: &str) -> Result<Vec<u8>, JsValue> {
    log_info(&format!("🟡 Starting image conversion"));
    log_debug(&format!("📥 Raw input length: {} bytes", input.len()));
    log_debug(&format!("🎯 Requested output format: {}", format));

    let img: DynamicImage = match load_from_memory(input) {
        Ok(image) => {
            log_info("✅ Image loaded successfully");
            image
        },
        Err(e) => {
            let msg = format!("❌ Failed to decode image: {:?}", e);
            log_error(&msg);
            return Err(JsValue::from_str(&msg));
        }
    };

    let output_format = match format.to_lowercase().as_str() {
        "png" => ImageOutputFormat::Png,
        "jpeg" | "jpg" => ImageOutputFormat::Jpeg(80),
        "webp" => ImageOutputFormat::WebP,
        "bmp" => ImageOutputFormat::Bmp,
        "gif" => ImageOutputFormat::Gif,
        "tiff" => ImageOutputFormat::Tiff,
        unknown => {
            let msg = format!("❌ Unsupported format: {}", unknown);
            log_error(&msg);
            return Err(JsValue::from_str(&msg));
        }
    };

    let mut buffer: Vec<u8> = Vec::new();
    match img.write_to(&mut Cursor::new(&mut buffer), output_format) {
        Ok(_) => {
            log_info(&format!("✅ Conversion to '{}' successful", format));
            log_debug(&format!("📦 Output size: {} bytes", buffer.len()));
            Ok(buffer)
        },
        Err(e) => {
            let msg = format!("❌ Failed to encode image: {:?}", e);
            log_error(&msg);
            Err(JsValue::from_str(&msg))
        }
    }
}

#[inline]
fn log_info(msg: &str) {
    console::log_1(&JsValue::from_str(&format!("[INFO] {}", msg)));
}

#[inline(always)]
fn log_debug(_msg: &str) {
    #[cfg(debug_assertions)]
    console::log_1(&JsValue::from_str(&format!("[DEBUG] {}", _msg)));
}
#[inline]
fn log_error(msg: &str) {
    console::error_1(&JsValue::from_str(&format!("[ERROR] {}", msg)));
}