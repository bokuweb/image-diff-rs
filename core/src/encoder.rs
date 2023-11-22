use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EncodeOutput {
    pub buf: Vec<u8>,
}

pub fn encode(rgba: &[u8], width: u32, height: u32) -> Result<Vec<u8>, ImageDiffError> {
    encode_webp(rgba, width, height)
}
