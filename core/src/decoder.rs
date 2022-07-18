use std::path::Path;

use image::GenericImageView;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DecodeOutput {
    pub buf: Vec<u8>,
    pub dimensions: (u32, u32),
}

pub fn decode<P: AsRef<Path>>(path: P) -> Result<DecodeOutput, ImageDiffError> {
    let p = path.as_ref();
    let ext = p.extension().ok_or_else(|| {
        ImageDiffError::InputExtensionError(p.to_str().expect("should convert").to_string())
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
        None => Err(ImageDiffError::InputExtensionError("none".to_owned())),
    }
}

pub fn decode_buf(buf: &[u8]) -> Result<DecodeOutput, ImageDiffError> {
    match buf {
        // RIFF .... WEBP
        [b'R', b'I', b'F', b'F', _, _, _, _, b'W', b'E', b'B', b'P', ..] => {
            Ok(decode_webp_buf(buf).map_err(|_| {
                ImageDiffError::DecodeError(("Failed to decode as webp format").to_string())
            })?)
        }
        _ => {
            let opened = image::load_from_memory(buf)?;
            Ok(DecodeOutput {
                dimensions: opened.dimensions(),
                buf: opened.into_bytes(),
            })
        }
    }
}

#[test]
fn test_decode_webp_buf() {
    let buf = include_bytes!("../../fixtures/sample0.webp");
    let res = decode_buf(buf);
    assert!(res.is_ok());
    let decoded = res.unwrap();
    assert_eq!(decoded.dimensions.0, 800);
    assert_eq!(decoded.dimensions.1, 578);
}

#[test]
fn test_decode_png_buf() {
    let buf = include_bytes!("../../fixtures/sample0.png");
    let res = decode_buf(buf);
    assert!(res.is_ok());
    let decoded = res.unwrap();
    assert_eq!(decoded.dimensions.0, 800);
    assert_eq!(decoded.dimensions.1, 578);
}
