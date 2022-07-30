use image_diff_rs::DiffInput;

#[no_mangle]
pub fn diff(
    imga: &[u8],
    imgb: &[u8],
    output: &mut *const u8,
    output_length: &mut usize,
    output_width: &mut u32,
    output_height: &mut u32,
    diff_count: &mut u32,
) {
    let res = image_diff_rs::diff(&DiffInput {
        actual_buf: imga,
        expected_buf: imgb,
        threshold: None,
        include_anti_alias: None,
    });
    let res = res.expect("TODO:");
    let diff_img = res.diff_image;
    *output_length = diff_img.len();
    *output = diff_img.as_ptr();
    std::mem::forget(diff_img);
}
