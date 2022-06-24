use std::path::Path;

use image::GenericImageView;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EncodeOutput {
    pub buf: Vec<u8>,
}

pub fn encode<P: AsRef<Path>>(
    path: P,
    rgba: &[u8],
    width: u32,
    height: u32,
) -> Result<(), ImageDiffError> {
    Ok(encode_webp(path, rgba, width, height).expect("TODO:"))

    // let p = path.as_ref();
    // let ext = p.extension().ok_or_else(|| {
    //     ImageDiffError::ExtensionError(p.to_str().expect("should convert").to_string())
    // })?;
    //
    // match ext.to_str() {
    //     Some("webp") => Ok(decode_webp(path)?),
    //     Some(_) => {
    //         todo!(),
    //     }
    //     None => Err(ImageDiffError::ExtensionError(
    //         "Failed to detect extension.".to_owned(),
    //     )),
    // }
}
