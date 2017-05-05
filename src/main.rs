use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::PathBuf;

extern crate clap;
use clap::{App, Arg};

extern crate chariot_slp;
extern crate chariot_palette;

extern crate png;
use png::HasParameters;

fn main() {
    let matches = App::new("slp-to-png")
        .version("0.1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("Convert an SLP file to indexed/paletted PNGs")
        .arg(Arg::with_name("slp-path")
            .long("slp-path")
            .value_name("slp-path")
            .help("Filepath to the SLP to convert to PNGs")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("pal-path")
            .long("pal-path")
            .value_name("pal-path")
            .help("Filepath to the palette (bin) to use")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("output-path")
            .short("o")
            .long("output-path")
            .help("A directory to write the PNGs to")
            .required(true)
            .takes_value(true))
        .get_matches();

    let slp_path = matches.value_of("slp-path").unwrap();
    let slp = chariot_slp::SlpFile::read_from_file(slp_path, 1).expect(&format!("Failed to read SLP from {}", slp_path));

    let pal_path = matches.value_of("pal-path").unwrap();
    // FIXME: Convert `pal` from Vec<PaletteColor> to [u8]
    let pal = chariot_palette::read_from_file(pal_path).expect(&format!("Failed to read palette from {}", pal_path));

    let output_path = PathBuf::from(matches.value_of("output-path").unwrap());

    for (idx, shape) in slp.shapes.iter().enumerate() {
        let output_name = format!("output{}.png", idx+1);
        let output_path = output_path.join(output_name);
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&output_path)
            .expect(&format!("Failed to prepare '{}'", output_path.display()));

        let ref mut w = BufWriter::new(f);

        // TODO: Convert slp to pal indices and deal with skips
        let data = vec![
            0,1,
            2,3
        ];

        let mut writer = {
            let mut encoder = png::Encoder::new(w, shape.header.width, shape.header.height);
            encoder.set(png::ColorType::Indexed).set(png::BitDepth::Eight);
            encoder.write_header().expect("Failed to write header for 'test.png'")
        };

        writer.write_chunk(png::chunk::PLTE, &pal).expect("Failed to write pal to 'test.png'");
        writer.write_image_data(&data).expect("Failed to write image data to 'test.png'");
    }
}