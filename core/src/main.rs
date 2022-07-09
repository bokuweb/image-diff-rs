use image_diff_rs::*;

pub fn main() {
    // decode_webp(data);
    diff(
        CompareInput::new("../fixtures/sample0.webp", "../fixtures/sample1.webp")
            .diff_filename("./diff.webp"),
    );
    // compare("./fixtures/sample0.png", "./fixtures/sample1.png");
}
