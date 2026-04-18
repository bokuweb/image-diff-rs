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
            let _span = tracing::info_span!("decode_webp", bytes = buf.len()).entered();
            Ok(decode_webp_buf(buf).map_err(|_| {
                ImageDiffError::Decode(("Failed to decode as webp format").to_string())
            })?)
        }
        _ => {
            let _span = tracing::info_span!("decode_image_crate", bytes = buf.len()).entered();

            let opened = {
                let _s = tracing::info_span!("load_from_memory").entered();
                image::load_from_memory(buf)
            };
            match opened {
                Ok(img) => {
                    let dims = img.dimensions();
                    let buf_rgba = {
                        let _s = tracing::info_span!(
                            "to_rgba8",
                            width = dims.0,
                            height = dims.1
                        )
                        .entered();
                        img.to_rgba8().to_vec()
                    };
                    Ok(DecodeOutput {
                        dimensions: dims,
                        buf: buf_rgba,
                    })
                }
                Err(e) => Err(ImageDiffError::Decode(e.to_string())),
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
