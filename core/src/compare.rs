use std::os::raw::*;
use std::path::Path;

use pixelmatch::*;

extern "C" {
    fn version() -> c_int;
    fn decode(
        data: *const c_uchar,
        size: usize,
        width: &mut c_int,
        height: &mut c_int,
    ) -> *const c_uchar;
}

pub fn webp_version() -> i32 {
    unsafe { version() }
}

pub fn decode_webp(data: &[u8]) -> Result<Vec<u8>, ()> {
    let mut w = 0;
    let mut h = 0;

    let result = unsafe { decode(data.as_ptr(), data.len(), &mut w, &mut h) };
    if result.is_null() {
        //return Err(());
        todo!()
    }
    dbg!(w, h, result);

    Ok(unsafe { std::slice::from_raw_parts(result, w as usize * h as usize * 4) }.to_vec())
}

pub struct DiffOutput {
    pub diff_count: usize,
    pub diff_image: Vec<u8>,
}

pub fn compare_files<P: AsRef<Path>>(img_path1: P, img_path2: P) -> Result<DiffOutput, ()> {
    todo!()
}

pub fn compare(img1: &[u8], img2: &[u8], dimensions: (u32, u32)) -> Result<DiffOutput, ()> {
    let result = pixelmatch(
        img1,
        img2,
        dimensions,
        Some(PixelmatchOption {
            threshold: 0.1,
            include_anti_alias: true,
            ..PixelmatchOption::default()
        }),
    )
    .expect("TODO:");
    Ok(DiffOutput {
        diff_count: result.diff_count,
        diff_image: result.diff_image,
    })
}
