use image::GenericImageView;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DecodeOutput {
    pub buf: Vec<u8>,
    pub dimensions: (u32, u32),
}

pub fn decode_buf(buf: &[u8]) -> Result<DecodeOutput, ImageDiffError> {
    match buf {
        // RIFF .... WEBP
        [b'R', b'I', b'F', b'F', _, _, _, _, b'W', b'E', b'B', b'P', ..] => {
            Ok(decode_webp_buf(buf).map_err(|_| {
                ImageDiffError::Decode(("Failed to decode as webp format").to_string())
            })?)
        }
        _ => {
            let i = image::load_from_memory(buf);
            if let Ok(opened) = i {
                Ok(DecodeOutput {
                    dimensions: opened.dimensions(),
                    buf: opened.to_rgba8().to_vec(),
                })
            } else {
                Err(ImageDiffError::Decode(i.unwrap_err().to_string()))
            }
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
