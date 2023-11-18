use std::path::Path;

mod compare;
mod decoder;
mod encoder;
mod error;
mod webp;

pub use compare::*;
pub use decoder::*;
pub use encoder::*;
pub use error::*;
pub use webp::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DiffFilesInput<P: AsRef<Path> + Clone> {
    pub actual_filename: P,
    pub expected_filename: P,
    pub diff_filename: P,
    pub threshold: Option<f32>,
    pub include_anti_alias: Option<bool>,
    pub webp_quality: Option<f32>,
}

impl<P: AsRef<Path> + Clone> DiffFilesInput<P> {
    pub fn new(actual_filename: P, expected_filename: P, diff_filename: P) -> Self {
        Self {
            actual_filename,
            expected_filename,
            diff_filename,
            threshold: None,
            include_anti_alias: None,
            webp_quality: None,
        }
    }
}

pub fn diff_files<P: AsRef<Path> + Clone>(
    input: DiffFilesInput<P>,
) -> Result<DiffOutput, ImageDiffError> {
    use std::{fs::File, io::Write};

    let result = compare(&CompareInput {
        actual_filename: input.actual_filename,
        expected_filename: input.expected_filename,
        diff_filename: input.diff_filename.clone(),
        threshold: input.threshold,
        include_anti_alias: input.include_anti_alias,
    })?;

    let diff_encoded = encode(&result.diff_image, result.width, result.height)?;

    let mut file = File::create(&input.diff_filename)?;
    file.write_all(&diff_encoded)?;
    file.flush()?;

    Ok(result)
}

pub struct DiffInput<'a> {
    pub actual_buf: &'a [u8],
    pub expected_buf: &'a [u8],
    pub threshold: Option<f32>,
    pub include_anti_alias: Option<bool>,
}

impl<'a> DiffInput<'a> {
    pub fn new(actual_buf: &'a [u8], expected_buf: &'a [u8]) -> Self {
        Self {
            actual_buf,
            expected_buf,
            threshold: None,
            include_anti_alias: None,
        }
    }
}

pub fn diff(input: &DiffInput) -> Result<DiffOutput, ImageDiffError> {
    let img1 = decode_buf(input.actual_buf)?;
    let img2 = decode_buf(input.expected_buf)?;
    let result = compare_buf(
        &img1.buf,
        &img2.buf,
        img1.dimensions,
        CompareOption {
            threshold: input.threshold.unwrap_or_default(),
            enable_anti_alias: input.include_anti_alias.unwrap_or_default(),
        },
    )?;
    Ok(result)
}
