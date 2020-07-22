use image;
use rqrr;
#[macro_use]
extern crate clap;

fn main() {
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
        let img = image::open(matches.value_of("file").unwrap_or("test.png")).unwrap().to_luma();
        let mut img = rqrr::PreparedImage::prepare_from_greyscale(img.width() as usize, img.height() as usize, |x, y: usize| -> u8 { img.get_pixel(x as u32, y as u32)[0] });
        let grids = img.detect_grids();
        println!("There are {} grids", grids.len());
        for grid in grids.iter() {
            let (meta, content) = grid.decode().unwrap();
            println!("{:?}", meta);
            println!("{}", content);
        }
    } else {println!("Subcommand required");}
    
}
