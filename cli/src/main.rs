use image_diff_rs::*;
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

// fn decode_webp<P: AsRef<Path>>(path: P) -> Result<DecodeOutput, ImageDiffError> {
//     let mut file = std::fs::File::open(path.as_ref()).unwrap();
//     let mut buf = Vec::new();
//     let _ = file.read_to_end(&mut buf);
//     decode_webp_buf(&buf).map_err(|_| {
//         ImageDiffError::Decode(path.as_ref().to_str().expect("should convert").to_string())
//     })
// }

pub fn decode<P: AsRef<Path>>(_path: P) -> Result<DecodeOutput, ImageDiffError> {
    todo!();
    // let p = path.as_ref();
    // let ext = p.extension().ok_or_else(|| {
    //     // ImageDiffError::InputExtension(p.to_str().expect("should convert").to_string())
    // })?;
    //
    // match ext.to_str() {
    //     Some("webp") => Ok(decode_webp(path)?),
    //     Some(_) => {
    //         let opened = image::open(path.as_ref())?;
    //         Ok(DecodeOutput {
    //             dimensions: opened.dimensions(),
    //             buf: opened.into_bytes(),
    //         })
    //     }
    //     None => {
    //         // Err(ImageDiffError::InputExtension("none".to_owned()));
    //     }
    // }
}

// fn compare<P: AsRef<Path>>(
//     input: &CompareInput<P>,
// ) -> Result<image_diff_rs::DiffOutput, image_diff_rs::ImageDiffError> {
//     let decoded1 = decode(input.actual_filename.as_ref())?;
//     let decoded2 = decode(input.expected_filename.as_ref())?;
//
//     image_diff_rs::compare_buf(
//         &decoded1.buf,
//         &decoded2.buf,
//         decoded1.dimensions,
//         CompareOption {
//             threshold: input.threshold.unwrap_or_default(),
//             enable_anti_alias: input.include_anti_alias.unwrap_or_default(),
//         },
//     )
// }

pub fn diff_files<P: AsRef<Path> + Clone>(
    _input: DiffFilesInput<P>,
) -> Result<image_diff_rs::DiffOutput, image_diff_rs::ImageDiffError> {
    todo!();
    // use std::{fs::File, io::Write};
    //
    // let result = compare(&CompareInput {
    //     actual_filename: input.actual_filename,
    //     expected_filename: input.expected_filename,
    //     diff_filename: input.diff_filename.clone(),
    //     threshold: input.threshold,
    //     include_anti_alias: input.include_anti_alias,
    // })?;
    //
    // let diff_encoded = image_diff_rs::encode(&result.diff_image, result.width, result.height)?;
    //
    // let mut file = File::create(&input.diff_filename)?;
    // file.write_all(&diff_encoded)?;
    // file.flush()?;
    //
    // Ok(result)
} //
