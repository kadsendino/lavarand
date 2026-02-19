use std::io::BufWriter;
use std::io::Write;
use fast_image_resize::{Resizer};
use fast_image_resize::images::Image;
use image::ImageReader;
use fast_image_resize::IntoImageView;

pub fn resize_image(
    input_path: &str,
    dst_width: u32,
    dst_height: u32,
    result_buffer: &mut BufWriter<Vec<u8>>,
) -> image::ImageResult<()> {

    // Load source image
    let src_image = ImageReader::open(input_path)?.decode()?;

    // Create destination image container
    let mut dst_image = Image::new(
        dst_width,
        dst_height,
        src_image.pixel_type().unwrap(),
    );

    // Resize
    let mut resizer = Resizer::new();
    resizer.resize(&src_image, &mut dst_image, None).unwrap();

    // Write raw bytes (RGBA or RGB depending on source) into buffer
    result_buffer.write_all(dst_image.buffer())?;

    Ok(())
}

