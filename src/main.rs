use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::PathBuf;

#[macro_use(value_t)]
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
        .arg(Arg::with_name("player")
            .long("player")
            .value_name("a value ranging from 1 to 8")
            .help("Player remap index (1..8)")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("output-path")
            .short("o")
            .long("output-path")
            .help("A directory to write the PNGs to")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("single-frame")
            .long("single-frame")
            .help("The frame index to extract (this is optional; if you do not provide this all frames will be extracted")
            .takes_value(true))
        .get_matches();

    let slp = {
        let slp_path = matches.value_of("slp-path").unwrap();
        let player_idx = {
            let v = matches.value_of("player").unwrap();
            let v = v.parse::<u8>().expect(&format!("Failed to parse {} into an integer value ranging from 1 to 8", v));

            if v > 8 {
                8
            } else if v == 0 {
                1
            } else {
                v
            }
        };

        chariot_slp::SlpFile::read_from_file(slp_path, player_idx).expect(&format!("Failed to read SLP from {}", slp_path))
    };

    let pal = {
        let pal_path = matches.value_of("pal-path").unwrap();
        let colors = chariot_palette::read_from_file(pal_path).expect(&format!("Failed to read palette from {}", pal_path));
        let mut rgb = vec![0u8; colors.len() * 3];

        for (index, color) in colors.iter().enumerate() {
            rgb[index * 3] = color.r;
            rgb[index * 3 + 1] = color.g;
            rgb[index * 3 + 2] = color.b;
        }

        rgb
    };

    let output_path = PathBuf::from(matches.value_of("output-path").unwrap());
    std::fs::create_dir_all(&output_path).expect(&format!("Failed to create output-path {}", output_path.display()));

    let single_frame_idx = {
        if matches.is_present("single-frame") {
            let v = value_t!(matches, "single-frame", usize).unwrap();
            if v > slp.shapes.len() - 1 {
                Some(slp.shapes.len() - 1)
            } else {
                Some(v)
            }
        } else {
            None
        }
    };

    for (idx, shape) in slp.shapes.iter().enumerate() {
        if let Some(index) = single_frame_idx {
            if idx != index {
                continue;
            }
        }

        let output_name = format!("output{}.png", idx);
        let output_path = output_path.join(output_name);
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&output_path)
            .expect(&format!("Failed to prepare '{}'", output_path.display()));

        let ref mut w = BufWriter::new(f);

        let mut writer = {
            let mut encoder = png::Encoder::new(w, shape.header.width, shape.header.height);
            encoder.set(png::ColorType::Indexed).set(png::BitDepth::Eight);
            encoder.write_header().expect(&format!("Failed to write header for '{}'", output_path.display()))
        };

        writer.write_chunk(png::chunk::PLTE, &pal).expect(&format!("Failed to write pal to '{}'", output_path.display()));
        writer.write_image_data(&shape.pixels).expect(&format!("Failed to write image data to '{}'", output_path.display()));
    }
}