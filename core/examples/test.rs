use image_diff_rs::*;

pub fn main() {
    diff_files(DiffFilesInput::new(
        "./fixtures/sample0.png",
        "./fixtures/sample1.png",
        "./diff.png",
    ));
}
