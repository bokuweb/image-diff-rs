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

pub struct DiffOption {
    pub threshold: Option<f32>,
    pub include_anti_alias: Option<bool>,
}

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
    Ok(result)
}
