use image::{ImageBuffer, Rgb};
use rqrr;
use log::{error, warn, info, debug, trace};

use crate::symbol::{Symbol, symbol_from_string};

fn symbols_from_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<Symbol> {
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
            Ok((meta, content)) => {
            },
            Err(e) => {info!("Failed to parse grid's content: {:?}", e);},
        }
    }
    output
}
