use image_diff_rs::*;

pub fn main() {
    let res = compare(&CompareInput {
        actual_filename: "../fixtures/sample0.webp",
        expected_filename: "../fixtures/sample1.webp",
        diff_filename: "../diff0.webp",
        threshold: Some(0.01),
        include_anti_alias: Some(true),
    })
    .expect("should compare succeed.");
    println!("diff count = {}", res.diff_count);
}
