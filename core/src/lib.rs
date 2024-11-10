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
    let img1 = decode_buf(actual.as_ref())?;
    let img2 = decode_buf(expected.as_ref())?;

    let w = std::cmp::max(img1.dimensions.0, img2.dimensions.0);
    let h = std::cmp::max(img1.dimensions.1, img2.dimensions.1);

    // expand ig size is not match.
    let expanded1 = expander::expand(img1.buf, img1.dimensions, w, h);
    let expanded2 = expander::expand(img2.buf, img2.dimensions, w, h);

    let result = compare_buf(
        &expanded1,
        &expanded2,
        (w, h),
        CompareOption {
            threshold: option.threshold.unwrap_or_default(),
            enable_anti_alias: option.include_anti_alias.unwrap_or_default(),
        },
    )?;

    match result {
        DiffOutput::Unmacthed {
            diff_count,
            diff_image,
            width,
            height,
        } => Ok(DiffOutput::Unmacthed {
            diff_count,
            diff_image: encode(&diff_image, width, height)?,
            width,
            height,
        }),
        DiffOutput::Matched { width, height } => Ok(DiffOutput::Matched { width, height }),
    }
}
