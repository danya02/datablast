use qrcode::QrCode;
use qrcode::types::QrError;
use image::{Rgb, ImageBuffer};

use crate::symbol;

enum QrEncodeError {
    EncodingLibError(QrError),
}

type QrEncodeResult = Result<ImageBuffer<Rgb<u8>, Vec<u8>>, QrEncodeError>;

pub fn symbol_to_qrcode(symb: symbol::Symbol) -> QrEncodeResult {
    string_to_qrcode(symb.to_str())
}

pub fn string_to_qrcode(data: String) -> QrEncodeResult {
    let code = QrCode::new(data);
    let to_render;
    match code {
        Ok(res) => {to_render = res;},
        Err(error) => {return Err(QrEncodeError::EncodingLibError(error));}
    }
    let img = to_render.render::<Rgb<u8>>().build();
//                    .light_color(Rgb([255, 255, 255])).build();
    Ok(img)
}
