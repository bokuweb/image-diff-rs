use std::path::Path;

use pixelmatch::*;

use super::*;

pub struct DiffOutput {
    pub diff_count: usize,
    pub diff_image: Vec<u8>,
}

pub struct CompareInput<P: AsRef<Path>> {
    pub actual_filename: P,
    pub expected_filename: P,
    pub diff_filename: Option<P>,
    pub threshold: Option<usize>,
    pub include_anti_alias: Option<bool>,
}

impl<P: AsRef<Path>> CompareInput<P> {
    pub fn new(actual_filename: P, expected_filename: P) -> Self {
        Self {
            actual_filename,
            expected_filename,
            diff_filename: None,
            threshold: None,
            include_anti_alias: None,
        }
    }

    pub fn diff_filename(&mut self, p: P) -> &mut Self {
        self.diff_filename = Some(p);
        self
    }
}

pub fn compare<P: AsRef<Path>>(input: &CompareInput<P>) -> Result<DiffOutput, ()> {
    let decoded1 = decode(input.actual_filename.as_ref()).expect("todo");
    let decoded2 = decode(input.expected_filename.as_ref()).expect("todo");

    compare_buf(&decoded1.buf, &decoded2.buf, decoded1.dimensions)
}

pub fn compare_buf(img1: &[u8], img2: &[u8], dimensions: (u32, u32)) -> Result<DiffOutput, ()> {
    let result = pixelmatch(
        img1,
        img2,
        dimensions,
        Some(PixelmatchOption {
            threshold: 0.1,
            include_anti_alias: true,
            ..PixelmatchOption::default()
        }),
    )
    .expect("TODO:");
    dbg!(&result.diff_count);
    Ok(DiffOutput {
        diff_count: result.diff_count,
        diff_image: result.diff_image,
    })
}
