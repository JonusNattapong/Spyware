use base64::{Engine as _, engine::general_purpose};
use image::ImageFormat;
use screenshots;
use std::io::Cursor;

pub fn capture_screenshot() -> String {
    if let Ok(screens) = screenshots::Screen::all() {
        if let Some(screen) = screens.first() {
            if let Ok(image) = screen.capture() {
                let mut cursor = Cursor::new(Vec::new());
                image.write_to(&mut cursor, ImageFormat::Png).ok();
                let buf = cursor.into_inner();
                general_purpose::STANDARD.encode(buf)
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    }
}