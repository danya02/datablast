use qrcode::QrCode;
use qrcode::types::{QrError, Color};
use image::{Rgb, RgbImage};

use crate::symbol;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QrEncodeError {
    #[error("The QR encoding library returned this error: {0}")]
    EncodingLibError(QrError),
}

pub type QrEncodeResult = Result<RgbImage, QrEncodeError>;

/// Encode the information in this symbol as a QR code.
pub fn symbol_to_qrcode(symb: symbol::Symbol) -> QrEncodeResult {
    string_to_qrcode(symb.to_str())
}

/// Render this QR code onto an image.
fn qrcode_to_image(code: QrCode) -> RgbImage {
    let width = code.width() as u32;
    let mut img = RgbImage::new(width+8, width+8); // leaving 4 pixels for the quiet zone
    let colors = code.to_colors();
    let mut colors_iter = colors.iter();
    for x in 4..=(width+4) {
        for y in 4..=(width+4) {
            let pixel = colors_iter.next().unwrap_or(&Color::Light);
            img.put_pixel(x, y, pixel.select(Rgb([0,0,0]), Rgb([255,255,255])));
        }
    }
    img
}

/// Get an image of a QR code that encodes this string.
pub fn string_to_qrcode(data: String) -> QrEncodeResult {
    let code = QrCode::new(data);
    let to_render;
    match code {
        Ok(res) => {to_render = res;},
        Err(error) => {return Err(QrEncodeError::EncodingLibError(error));}
    }
    let img = qrcode_to_image(to_render);
    Ok(img)
}
