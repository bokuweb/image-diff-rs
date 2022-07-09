use image_diff_rs::*;

pub fn main() {
    diff(DiffInput::new(
        "./fixtures/sample0.png",
        "./fixtures/sample1.png",
        "./diff.png",
    ));
}
