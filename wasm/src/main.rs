pub fn main() {
    let data = include_bytes!("../../test_webp_js.webp");
    image_diff_rs::decode_buf(data);
    println!("Hello");

}

#[no_mangle]
pub fn compare() {
    println!("Hello2");
}
