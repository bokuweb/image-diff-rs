use std::path::Path;

use image::GenericImageView;

use super::*;

pub struct DecodeOutput {
    pub buf: Vec<u8>,
    pub dimensions: (u32, u32),
}
pub fn decode<P: AsRef<Path>>(path: P) -> Result<DecodeOutput, ImageDiffError> {
    let p = path.as_ref();
    let ext = p.extension().ok_or_else(|| {
        ImageDiffError::ExtensionError(p.to_str().expect("should convert").to_string())
    })?;

    match ext.to_str() {
        Some("webp") => Ok(decode_webp(path)?),
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
