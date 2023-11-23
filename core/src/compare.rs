use std::path::Path;

use pixelmatch::*;

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DiffOutput {
    pub diff_count: usize,
    pub diff_image: Vec<u8>,
    pub width: u32,
    pub height: u32,
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

pub fn compare<P: AsRef<Path>>(input: &CompareInput<P>) -> Result<DiffOutput, ImageDiffError> {
    let decoded1 = decode(input.actual_filename.as_ref())?;
    let decoded2 = decode(input.expected_filename.as_ref())?;

    compare_buf(
        &decoded1.buf,
        &decoded2.buf,
        decoded1.dimensions,
        CompareOption {
            threshold: input.threshold.unwrap_or_default(),
            enable_anti_alias: input.include_anti_alias.unwrap_or_default(),
        },
    )
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
    Ok(DiffOutput {
        diff_count: result.diff_count,
        diff_image: result.diff_image,
        width: dimensions.0,
        height: dimensions.1,
    })
}
