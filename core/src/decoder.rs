use std::path::Path;

use image::GenericImageView;

use super::*;

pub struct DecodeOutput {
    pub buf: Vec<u8>,
    pub dimensions: (u32, u32),
}
pub fn decode<P: AsRef<Path>>(path: P) -> Result<DecodeOutput, ImageDiffError> {
    let ext = path
        .as_ref()
        .extension()
        .ok_or(ImageDiffError::ExtensionError)?;

    match ext.to_str() {
        Some("webp") => Ok(decode_webp(path).expect("TODO:")),
        Some(_) => {
            let opened = image::open(path.as_ref())?;
            Ok(DecodeOutput {
                dimensions: opened.dimensions(),
                buf: opened.into_bytes(),
            })
        }
        None => todo!(),
    }
}
