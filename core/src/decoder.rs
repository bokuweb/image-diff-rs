use std::path::Path;

use image::GenericImageView;

use super::*;

pub struct DecodeOutput {
    pub buf: Vec<u8>,
    pub dimensions: (u32, u32),
}
pub fn decode<P: AsRef<Path>>(path: P) -> Result<DecodeOutput, ()> {
    let ext = path.as_ref().extension().ok_or(())?;

    match ext.to_str() {
        Some("webp") => decode_webp(path),
        Some(_) => {
            let opened = image::open(path.as_ref()).expect("todo");
            Ok(DecodeOutput {
                dimensions: opened.dimensions(),
                buf: opened.into_bytes(),
            })
        }
        None => todo!(),
    }
}
