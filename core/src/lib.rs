mod compare;
mod decoder;
mod encoder;
mod error;
mod expander;
mod pixelmatch;
mod webp;

pub use compare::*;
pub use decoder::*;
pub use encoder::*;
pub use error::*;
pub use webp::*;

/// Options for configuring the behavior of the `diff` function.
///
/// This struct allows users to set various parameters that influence how the image
/// difference calculation is performed.
///
#[derive(Debug, Default, Clone)]
pub struct DiffOption {
    /// specifying the sensitivity threshold for pixel differences.
    /// a lower value means more sensitivity to small changes.
    pub threshold: Option<f32>,
    /// that determines whether to include anti-aliased pixels in the diff calculation.
    pub include_anti_alias: Option<bool>,
    /// Output format for the diff image. `None` (default) keeps the legacy
    /// behaviour of encoding as WebP lossless.
    pub encode_format: Option<EncodeFormat>,
}

/// An unencoded image diff.
///
/// The RGBA buffer can be encoded with [`encode_diff`] after a caller has
/// inspected `diff_count`, `width`, and `height`. This is useful when a caller
/// applies its own acceptance threshold and does not need an image for accepted
/// differences.
#[derive(Debug, PartialEq, Clone)]
pub struct RgbaDiff {
    /// The total number of pixels that differ between the two images.
    pub diff_count: usize,
    /// The unencoded diff image as RGBA pixels.
    pub rgba: Vec<u8>,
    /// The width of the diff image.
    pub width: u32,
    /// The height of the diff image.
    pub height: u32,
}

/// Compares two encoded images without encoding the resulting diff image.
///
/// Unlike [`diff`], this function always decodes and compares the inputs,
/// including when their encoded bytes are identical. Call [`encode_diff`] only
/// when the RGBA visualization is needed.
pub fn diff_rgba(
    actual: impl AsRef<[u8]>,
    expected: impl AsRef<[u8]>,
    option: &DiffOption,
) -> Result<RgbaDiff, ImageDiffError> {
    let _span = tracing::info_span!(
        "image_diff_rgba",
        actual_bytes = actual.as_ref().len(),
        expected_bytes = expected.as_ref().len()
    )
    .entered();

    diff_rgba_inner(actual.as_ref(), expected.as_ref(), option)
}

fn diff_rgba_inner(
    actual: &[u8],
    expected: &[u8],
    option: &DiffOption,
) -> Result<RgbaDiff, ImageDiffError> {
    let img1 = {
        let _s = tracing::info_span!("decode_actual", bytes = actual.len()).entered();
        decode_buf(actual)?
    };
    let img2 = {
        let _s = tracing::info_span!("decode_expected", bytes = expected.len()).entered();
        decode_buf(expected)?
    };

    let width = std::cmp::max(img1.dimensions.0, img2.dimensions.0);
    let height = std::cmp::max(img1.dimensions.1, img2.dimensions.1);

    let (expanded1, expanded2) = {
        let _s = tracing::info_span!("expand", width, height).entered();
        let e1 = expander::expand(img1.buf, img1.dimensions, width, height);
        let e2 = expander::expand(img2.buf, img2.dimensions, width, height);
        (e1, e2)
    };

    let result = {
        let _s = tracing::info_span!(
            "compare_pixels",
            width,
            height,
            pixels = (width as u64) * (height as u64)
        )
        .entered();
        compare_buf(
            &expanded1,
            &expanded2,
            (width, height),
            CompareOption {
                threshold: option.threshold.unwrap_or_default(),
                enable_anti_alias: option.include_anti_alias.unwrap_or_default(),
            },
        )?
    };

    match result {
        DiffOutput::NotEq {
            diff_count,
            diff_image,
            width,
            height,
        } => Ok(RgbaDiff {
            diff_count,
            rgba: diff_image,
            width,
            height,
        }),
        DiffOutput::Eq => unreachable!("compare_buf always returns a diff image"),
    }
}

/// Encodes an [`RgbaDiff`] and returns the legacy [`DiffOutput`] shape.
pub fn encode_diff(diff: &RgbaDiff, format: EncodeFormat) -> Result<DiffOutput, ImageDiffError> {
    let encoded = {
        let _s = tracing::info_span!(
            "encode_diff",
            width = diff.width,
            height = diff.height,
            diff_count = diff.diff_count,
            format = ?format
        )
        .entered();
        encode_with(&diff.rgba, diff.width, diff.height, format)?
    };

    Ok(DiffOutput::NotEq {
        diff_count: diff.diff_count,
        diff_image: encoded,
        width: diff.width,
        height: diff.height,
    })
}

/// Compares two images and calculates the differences between them.
///
/// This function takes two images as byte slices and an options struct, and returns
/// a `Result` containing either the diff output or an error.
///
/// # Arguments
/// * `actual`: A bytes representing the first image to be compared.
/// * `expected`: A bytes representing the second image to be compared.
/// * `option`: A reference to a `DiffOption` struct that specifies additional options for the diff operation.
///
pub fn diff(
    actual: impl AsRef<[u8]>,
    expected: impl AsRef<[u8]>,
    option: &DiffOption,
) -> Result<DiffOutput, ImageDiffError> {
    let _span = tracing::info_span!(
        "image_diff",
        actual_bytes = actual.as_ref().len(),
        expected_bytes = expected.as_ref().len()
    )
    .entered();

    if actual.as_ref() == expected.as_ref() {
        let _s = tracing::info_span!("image_diff.short_circuit_eq").entered();
        return Ok(DiffOutput::Eq);
    }

    let result = diff_rgba_inner(actual.as_ref(), expected.as_ref(), option)?;
    encode_diff(&result, option.encode_format.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTUAL: &[u8] = include_bytes!("../../fixtures/sample0.webp");
    const EXPECTED: &[u8] = include_bytes!("../../fixtures/sample1.webp");

    #[test]
    fn rgba_diff_exposes_metadata_before_encoding() {
        let result = diff_rgba(
            ACTUAL,
            EXPECTED,
            &DiffOption {
                threshold: Some(0.01),
                include_anti_alias: Some(true),
                ..Default::default()
            },
        )
        .unwrap();

        assert_eq!(result.diff_count, 3454);
        assert_eq!((result.width, result.height), (800, 578));
        assert_eq!(result.rgba.len(), 800 * 578 * 4);
    }

    #[test]
    fn staged_webp_output_matches_diff() {
        let option = DiffOption {
            threshold: Some(0.01),
            include_anti_alias: Some(true),
            ..Default::default()
        };
        let rgba = diff_rgba(ACTUAL, EXPECTED, &option).unwrap();

        assert_eq!(
            encode_diff(&rgba, EncodeFormat::Webp).unwrap(),
            diff(ACTUAL, EXPECTED, &option).unwrap()
        );
    }

    #[test]
    fn staged_png_output_matches_diff() {
        let option = DiffOption {
            threshold: Some(0.01),
            include_anti_alias: Some(true),
            encode_format: Some(EncodeFormat::Png),
        };
        let rgba = diff_rgba(ACTUAL, EXPECTED, &option).unwrap();

        assert_eq!(
            encode_diff(&rgba, EncodeFormat::Png).unwrap(),
            diff(ACTUAL, EXPECTED, &option).unwrap()
        );
    }

    #[test]
    fn encoded_outputs_remain_byte_compatible() {
        fn fnv1a(bytes: &[u8]) -> u64 {
            bytes.iter().fold(0xcbf29ce484222325, |hash, byte| {
                (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
            })
        }

        for (format, expected_len, expected_hash) in [
            (EncodeFormat::Webp, 7460, 0x0ea7fdd2612e399f),
            (EncodeFormat::Png, 37581, 0xc751afd428297407),
        ] {
            let option = DiffOption {
                threshold: Some(0.01),
                include_anti_alias: Some(true),
                encode_format: Some(format),
            };
            let result = diff(ACTUAL, EXPECTED, &option).unwrap();
            let DiffOutput::NotEq {
                diff_count,
                diff_image,
                width,
                height,
            } = result
            else {
                panic!("fixture images must differ");
            };

            assert_eq!(diff_count, 3454);
            assert_eq!((width, height), (800, 578));
            assert_eq!(diff_image.len(), expected_len);
            assert_eq!(fnv1a(&diff_image), expected_hash);
        }
    }

    #[test]
    fn diff_keeps_identical_input_short_circuit() {
        assert_eq!(
            diff(ACTUAL, ACTUAL, &DiffOption::default()).unwrap(),
            DiffOutput::Eq
        );
    }
}
