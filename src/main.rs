// use lavarand::lavarand;
use std::io::Write;
use std::fs::File;
use std::io::BufWriter;

mod image_utils;
use image_utils::resize_image;

mod hash_function;
use hash_function::n_way_sha1;
use hash_function::n_way_sha256;

mod number_generator;
use number_generator::blum_blum_shub;

fn main() {
    // let args = std::env::args().collect::<Vec<String>>();

    let mut buffer = BufWriter::new(Vec::new());

    let _ = resize_image("data/lavalamp.jpg", 480, 640, &mut buffer);

    let bytes = buffer.into_inner().unwrap();

    let num_pixels = bytes.len();
    println!("Number of pixels: {}", num_pixels);

    //save as image .pgm
    let output_path = "data/lavalamp_resized.pgm";
    let mut file = File::create(output_path).unwrap();
    // Write PGM header
    file.write_all(format!("P6\n{} {}\n255\n", 480, 640).as_bytes()).unwrap();
    // Write pixel data
    file.write_all(&bytes).unwrap();

    let n = 7;
    let seed = n_way_sha1(&bytes, n);
    println!("Seed length: {} bytes", seed.len()); // 140 bytes
    println!("Seed (hex):");
    for byte in seed.iter() {
        print!("{:02x}", byte);
    }
    println!();

    let random_bytes = blum_blum_shub(&seed, 128);

    println!("Generated BBS bytes (hex):");
    for byte in &random_bytes {
        print!("{:02x}", byte);
    }
    println!();

    // let output_path = "data/lavalamp_resized.png";
    // let file = File::create(output_path).unwrap();
    // let image_buffer = BufWriter::new(file);
    // image_buffer.into_inner().unwrap().write_all(&buffer.into_inner().unwrap()).unwrap();
}
