mod compare;
mod decoder;
mod encoder;
mod error;
mod expander;
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
pub struct DiffOption {
    /// specifying the sensitivity threshold for pixel differences.
    /// a lower value means more sensitivity to small changes.
    pub threshold: Option<f32>,
    /// that determines whether to include anti-aliased pixels in the diff calculation.
    pub include_anti_alias: Option<bool>,
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

    let img1 = {
        let _s = tracing::info_span!(
            "decode_actual",
            bytes = actual.as_ref().len()
        )
        .entered();
        decode_buf(actual.as_ref())?
    };
    let img2 = {
        let _s = tracing::info_span!(
            "decode_expected",
            bytes = expected.as_ref().len()
        )
        .entered();
        decode_buf(expected.as_ref())?
    };

    let w = std::cmp::max(img1.dimensions.0, img2.dimensions.0);
    let h = std::cmp::max(img1.dimensions.1, img2.dimensions.1);

    // expand if size is not match.
    let (expanded1, expanded2) = {
        let _s = tracing::info_span!("expand", width = w, height = h).entered();
        let e1 = expander::expand(img1.buf, img1.dimensions, w, h);
        let e2 = expander::expand(img2.buf, img2.dimensions, w, h);
        (e1, e2)
    };

    let result = {
        let _s = tracing::info_span!(
            "compare_pixels",
            width = w,
            height = h,
            pixels = (w as u64) * (h as u64)
        )
        .entered();
        compare_buf(
            &expanded1,
            &expanded2,
            (w, h),
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
        } => {
            let encoded = {
                let _s = tracing::info_span!(
                    "encode_diff_webp",
                    width = width,
                    height = height,
                    diff_count
                )
                .entered();
                encode(&diff_image, width, height)?
            };
            Ok(DiffOutput::NotEq {
                diff_count,
                diff_image: encoded,
                width,
                height,
            })
        }
        DiffOutput::Eq => Ok(DiffOutput::Eq),
    }
}
