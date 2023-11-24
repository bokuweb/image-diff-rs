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

pub fn diff_files<P: AsRef<Path> + Clone>(
    input: DiffFilesInput<P>,
) -> Result<image_diff_rs::DiffOutput, image_diff_rs::ImageDiffError> {
    use std::{fs::File, io::Write};

    let result = image_diff_rs::compare(&image_diff_rs::CompareInput {
        actual_filename: input.actual_filename,
        expected_filename: input.expected_filename,
        diff_filename: input.diff_filename.clone(),
        threshold: input.threshold,
        include_anti_alias: input.include_anti_alias,
    })?;

    let diff_encoded = image_diff_rs::encode(&result.diff_image, result.width, result.height)?;

    let mut file = File::create(&input.diff_filename)?;
    file.write_all(&diff_encoded)?;
    file.flush()?;

    Ok(result)
}
