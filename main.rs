use anyhow::Result;
use image::{guess_format, ImageFormat};
use std::fs::File;
use std::io::Read;
use zune_jpeg::JpegDecoder;

fn main() -> Result<()> {
    let mut file = File::open("./exiftool_output.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let format = guess_format(&buffer)?;
    assert_eq!(format, ImageFormat::Jpeg);

    let mut decoder = JpegDecoder::new(&buffer);
    let _ = decoder
        .decode()
        .map_err(|e| println!("Error in decode: {}", e));

    Ok(())
}
