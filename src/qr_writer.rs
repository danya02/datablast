use qrcode::{QrCode, QrError};
use image::Rgb;

enum QrEncodeError {
    EncodingLibError(QrError),
}

type QrEncodeResult = Result<ImageBuffer<Rgb<u8>, Vec<u8>>, QrEncodeError>;

pub fn symbol_to_qrcode(symb: symbol::Symbol) -> QrEncodeResult {
    string_to_qrcode(symb.to_str())?
}

pub fn string_to_qrcode(data: String) -> QrEncodeResult {
    let code = QrCode::new(data);
    match code {
        Ok(res) => {let code = res;},
        Err(error) => {return Err(QrEncodeError::EncodingLibError(error));}
    }
    Ok(code.build())
}
