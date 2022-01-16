use image_diff_rs::*;

pub fn main() {
    dbg!(version());
    let data = include_bytes!("../../test_webp_js.webp");
    decode_webp(data);
}
