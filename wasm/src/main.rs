pub fn main() {
    println!("{}", image_diff_rs::version());
    let data = include_bytes!("../../test_webp_js.webp");
    image_diff_rs::decode_webp(data);
}
