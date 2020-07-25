#[macro_use]
extern crate clap;

extern crate log;
use log::{error, warn, info, debug, trace};
extern crate simple_logger;

mod symbol;
mod qr_reader;
mod qr_writer;

use image::open;

fn main() {
    simple_logger::init().unwrap();
    let symb = symbol::symbol_from_string("test".to_string());
    let matches = clap_app!(myapp =>
        (version: "0.0")
        (about: "Datablast manipulation")
        (@subcommand qrread =>
            (about: "tests the QR-code symbol reader")
            (@arg file: -f +takes_value "file to read qr codes from")
        )
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("qrtest") {
        let filename = matches.value_of("file").expect("file name required");
        info!("Loading image {} ...", filename);
        let img = open(filename).expect("image invalid").to_rgb();
        let symbols = qr_reader::symbols_from_image(img);
        for symb in symbols.iter() {info!("Found symbol: {:?}", symb);}
    } else {println!("Subcommand required");}
    
}
