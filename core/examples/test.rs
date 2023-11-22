use image_diff_rs::*;

pub fn main() {
    diff_files(DiffFilesInput::new(
        "./fixtures/sample0.webp",
        "./fixtures/sample1.webp",
        "./diff.webp",
    ));
}
