use image_diff_rs::*;

pub fn main() {
    dbg!(version());
    let data = include_bytes!("../../test_webp_js.webp");
    // decode_webp(data);
    compare(
        CompareInput::new("./fixtures/sample0.webp", "./fixtures/sample1.webp")
            .diff_filename("./diff.png"),
    );
    // compare("./fixtures/sample0.png", "./fixtures/sample1.png");
}
