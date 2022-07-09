use std::path::Path;

mod compare;
mod decoder;
mod encoder;
mod error;
mod types;
mod webp;

pub use compare::*;
pub use decoder::*;
pub use encoder::*;
pub use error::*;
pub use types::*;
pub use webp::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DiffInput<P: AsRef<Path> + Clone> {
    pub actual_filename: P,
    pub expected_filename: P,
    pub diff_filename: P,
    pub threshold: Option<f32>,
    pub include_anti_alias: Option<bool>,
}

impl<P: AsRef<Path> + Clone> DiffInput<P> {
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

pub fn diff<P: AsRef<Path> + Clone>(input: DiffInput<P>) -> Result<DiffOutput, ImageDiffError> {
    use std::{fs::File, io::Write};

    let result = compare(&CompareInput {
        actual_filename: input.actual_filename,
        expected_filename: input.expected_filename,
        diff_filename: input.diff_filename.clone(),
        threshold: input.threshold,
        include_anti_alias: input.include_anti_alias,
    })?;

    let ext = input
        .diff_filename
        .as_ref()
        .clone()
        .extension()
        .ok_or_else(|| ImageDiffError::OutputExtensionError("none".to_string()))?;

    let diff_encoded = encode(
        ext.to_str().unwrap_or("png"),
        &result.diff_image,
        result.width,
        result.height,
        70.0,
    )?;

    let mut file = File::create(&input.diff_filename)?;
    file.write_all(&diff_encoded)?;
    file.flush()?;

    Ok(result)
}

pub fn diff_without_saving<P: AsRef<Path>>(
    input: &CompareInput<P>,
) -> Result<DiffOutput, ImageDiffError> {
    let result = compare(input)?;
    let ext = input
        .diff_filename
        .as_ref()
        .extension()
        .ok_or_else(|| ImageDiffError::OutputExtensionError("none".to_string()))?;

    let diff_encoded = encode(
        ext.to_str().unwrap_or("png"),
        &result.diff_image,
        result.width,
        result.height,
        70.0,
    )?;

    Ok(result)
}
