#[macro_use]
extern crate clap;

extern crate log;
use log::{error, warn, info, debug, trace};
extern crate simple_logger;

mod symbol;
mod qr_reader;

fn main() {
    simple_logger::init().unwrap();
    let symb = symbol::symbol_from_string("test".to_string());
    let matches = clap_app!(myapp =>
        (version: "0.0")
        (about: "Datablast manipulation")
        (@subcommand qrtest =>
            (about: "tests the QR-code reader")
            (version: "1.3")
            (@arg file: -f +takes_value "file to read qr code from")
        )
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("qrtest") {
        println!("Selected qrtest");
    } else {println!("Subcommand required");}
    
}
