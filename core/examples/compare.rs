use image_diff_rs::*;

pub fn main() {
    let imga = std::fs::read("./fixtures/sample0.webp").unwrap();
    let imgb = std::fs::read("./fixtures/sample1.webp").unwrap();

    let result = diff(
        imga,
        imgb,
        &DiffOption {
            threshold: Some(0.01),
            include_anti_alias: Some(true),
        },
    )
    .unwrap();

    if let DiffOutput::NotEq { diff_count, .. } = result {
        assert_eq!(diff_count, 3454);
    } else {
        unreachable!()
    }
}
