use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EncodeOutput {
    pub buf: Vec<u8>,
}

pub fn encode(
    ext: &str,
    rgba: &[u8],
    width: u32,
    height: u32,
    quality: f32,
) -> Result<Vec<u8>, ImageDiffError> {
    match ext {
        "webp" => Ok(encode_webp(rgba, width, height, quality)?),
        "png" => Ok(vec![]),
        _ => Err(ImageDiffError::OutputExtensionError(ext.to_string())),
    }
}
