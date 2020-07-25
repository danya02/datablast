use image::{ImageBuffer, Rgb};
use rqrr;
use log::{error, warn, info, debug, trace};

use crate::symbol::{Symbol, symbol_from_string};

pub fn symbols_from_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<Symbol> {
    let mut output = Vec::new();
    for content in strings_from_image(img).iter() {
        match symbol_from_string(content.to_string()) {
            Ok(symbol) => output.push(symbol),
            Err(error) => warn!("This content could not be parsed as a symbol: {:?} The error was: {:?}", content, error),
        } 
    }
    output
}

fn strings_from_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<String> {
    let mut output = Vec::new();
    let mut prep_img = rqrr::PreparedImage::prepare_from_greyscale(img.width() as usize, img.height() as usize, |x, y: usize| -> u8 { img.get_pixel(x as u32, y as u32)[0] });
    let grids = prep_img.detect_grids();
    for grid in grids.iter() {
        match grid.decode() {
            Ok((_meta, content)) => {
                output.push(content);
            },
            Err(e) => {info!("Failed to parse grid's content: {:?}", e);},
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use image::open;
    use crate::symbol::{Symbol, MetaSymbol, ContentSymbol};
    use crate::qr_reader::symbols_from_image;
    fn get_single_symbol(name: &str) -> Symbol {
        let img = open(name).unwrap();
        let mut symb = symbols_from_image(img.to_rgb());
        assert_eq!(symb.len(), 1);
        symb.remove(0)
    }

    #[test]
    fn test_read_metasymb() {
        let symb = get_single_symbol("test_data/metasymb1.png");
        assert_eq!(symb, Symbol::Meta(MetaSymbol { ver:0, frames:1000, cur_frame:5, content_len: vec![16384, 750], sha3: "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a".to_string(), name: "test.bin".to_string() }));
    }

    #[test]
    fn test_read_contentsymb() {
        let symb = get_single_symbol("test_data/contentsymb1.png");
        assert_eq!(symb, Symbol::Content(ContentSymbol {sequence: 0xff, index: 0xaaaa, data: b"HelloWorld!".to_vec() } ));
    }
}
