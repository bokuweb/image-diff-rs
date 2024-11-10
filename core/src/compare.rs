use std::path::Path;

use pixelmatch::*;

use super::*;

/// The output produced by the `diff` function.
///
/// Contains detailed information about the differences between two images.
///
#[derive(Debug, PartialEq, Clone)]
pub enum DiffOutput {
    Unmacthed {
        /// The total number of pixels that differ between the two images.
        diff_count: usize,
        /// A `Vec<u8>` containing the diff image data (WebP format).
        diff_image: Vec<u8>,
        /// The width of the diff image.
        width: u32,
        /// The height of the diff image.
        height: u32,
    },
    Matched {
        /// The width of the diff image.
        width: u32,
        /// The height of the diff image.
        height: u32,
    },
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CompareInput<P: AsRef<Path>> {
    pub actual_filename: P,
    pub expected_filename: P,
    pub diff_filename: P,
    pub threshold: Option<f32>,
    pub include_anti_alias: Option<bool>,
}

impl<P: AsRef<Path>> CompareInput<P> {
    pub fn new(actual_filename: P, expected_filename: P, diff_filename: P) -> Self {
        Self {
            actual_filename,
            expected_filename,
            diff_filename,
            threshold: None,
            include_anti_alias: None,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CompareOption {
    pub threshold: f32,
    pub enable_anti_alias: bool,
}

pub fn compare_buf(
    img1: &[u8],
    img2: &[u8],
    dimensions: (u32, u32),
    opt: CompareOption,
) -> Result<DiffOutput, ImageDiffError> {
    if img1 == img2 {
        return Ok(DiffOutput::Matched {
            width: dimensions.0,
            height: dimensions.1,
        });
    }

    let result = pixelmatch(
        img1,
        img2,
        dimensions,
        Some(PixelmatchOption {
            threshold: opt.threshold,
            include_anti_alias: true,
            ..PixelmatchOption::default()
        }),
    )
    .expect("pixelmatch should succeed, but it panicked. This appears to be a bug");

    Ok(DiffOutput::Unmacthed {
        diff_count: result.diff_count,
        diff_image: result.diff_image,
        width: dimensions.0,
        height: dimensions.1,
    })
}
