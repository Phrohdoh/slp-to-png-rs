use std::fs::OpenOptions;
use std::io::BufWriter;

extern crate png;
use png::HasParameters;

fn main() {
    let f = OpenOptions::new()
        .write(true)
        .create(true)
        .open("test.png")
        .expect("Failed to prepare 'test.png'");

    let ref mut w = BufWriter::new(f);

    let pal = vec![
        255, 0, 0,
        0, 255, 0,
        0, 0, 255,
        255, 0, 255,
    ];

    let data = vec![
        0,1,
        2,3
    ];

    let mut writer = {
        let mut encoder = png::Encoder::new(w, 2, 2);
        encoder.set(png::ColorType::Indexed).set(png::BitDepth::Eight);
        encoder.write_header().expect("Failed to write header for 'test.png'")
    };

    writer.write_chunk(png::chunk::PLTE, &pal).expect("Failed to write pal to 'test.png'");
    writer.write_image_data(&data).expect("Failed to write image data to 'test.png'");
}