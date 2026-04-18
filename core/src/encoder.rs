use super::*;
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder};

#[derive(Debug, Clone, PartialEq)]
pub struct EncodeOutput {
    pub buf: Vec<u8>,
}

/// Output format for the diff image.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum EncodeFormat {
    /// WebP lossless (default). Smaller + faster to encode for diff images
    /// (mostly solid colour + a few red pixels), but uses a custom libwebp
    /// build that is linked into this crate.
    #[default]
    Webp,
    /// PNG via the `image` crate. Larger + slower but matches the original
    /// `reg-cli` JS implementation's output.
    Png,
}

/// Legacy encoder: always WebP. Kept for source-compat with callers that
/// don't care about the output format.
pub fn encode(rgba: &[u8], width: u32, height: u32) -> Result<Vec<u8>, ImageDiffError> {
    encode_with(rgba, width, height, EncodeFormat::Webp)
}

/// Encode the diff image in the requested format.
pub fn encode_with(
    rgba: &[u8],
    width: u32,
    height: u32,
    format: EncodeFormat,
) -> Result<Vec<u8>, ImageDiffError> {
    match format {
        EncodeFormat::Webp => {
            let _span = tracing::info_span!(
                "encode_webp_ffi",
                width,
                height,
                rgba_bytes = rgba.len()
            )
            .entered();
            encode_webp(rgba, width, height)
        }
        EncodeFormat::Png => {
            let _span = tracing::info_span!(
                "encode_png",
                width,
                height,
                rgba_bytes = rgba.len()
            )
            .entered();
            let mut out: Vec<u8> = Vec::with_capacity(rgba.len() / 4);
            PngEncoder::new(&mut out)
                .write_image(rgba, width, height, ExtendedColorType::Rgba8)
                .map_err(|e| ImageDiffError::Encode(e.to_string()))?;
            Ok(out)
        }
    }
}
