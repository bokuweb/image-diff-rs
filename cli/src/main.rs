use std::path::Path;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DiffFilesInput<P: AsRef<Path> + Clone> {
    pub actual_filename: P,
    pub expected_filename: P,
    pub diff_filename: P,
    pub threshold: Option<f32>,
    pub include_anti_alias: Option<bool>,
}

impl<P: AsRef<Path> + Clone> DiffFilesInput<P> {
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

pub fn main() {}

/// The output produced by the `diff` function.
///
/// Contains detailed information about the differences between two images.
///
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DiffOutput {
    /// The total number of pixels that differ between the two images.
    pub diff_count: usize,
    /// A `Vec<u8>` containing the diff image data.
    pub diff_image: Vec<u8>,
    /// The width of the diff image.
    pub width: u32,
    /// The height of the diff image.
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

// fn compare<P: AsRef<Path>>(input: &CompareInput<P>) -> Result<DiffOutput, ImageDiffError> {
//     let decoded1 = decode(input.actual_filename.as_ref())?;
//     let decoded2 = decode(input.expected_filename.as_ref())?;
//
//     compare_buf(
//         &decoded1.buf,
//         &decoded2.buf,
//         decoded1.dimensions,
//         CompareOption {
//             threshold: input.threshold.unwrap_or_default(),
//             enable_anti_alias: input.include_anti_alias.unwrap_or_default(),
//         },
//     )
// }
//
// pub fn diff_files<P: AsRef<Path> + Clone>(
//     input: DiffFilesInput<P>,
// ) -> Result<image_diff_rs::DiffOutput, image_diff_rs::ImageDiffError> {
//     use std::{fs::File, io::Write};
//
//     let result = image_diff_rs::compare(&image_diff_rs::CompareInput {
//         actual_filename: input.actual_filename,
//         expected_filename: input.expected_filename,
//         diff_filename: input.diff_filename.clone(),
//         threshold: input.threshold,
//         include_anti_alias: input.include_anti_alias,
//     })?;
//
//     let diff_encoded = image_diff_rs::encode(&result.diff_image, result.width, result.height)?;
//
//     let mut file = File::create(&input.diff_filename)?;
//     file.write_all(&diff_encoded)?;
//     file.flush()?;
//
//     Ok(result)
// }
//
