use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::os::raw::*;
use std::path::Path;

use super::*;

extern "C" {
    fn decode(
        data: *const c_uchar,
        size: usize,
        width: &mut c_int,
        height: &mut c_int,
    ) -> *const c_uchar;
    fn encode(
        rgba: *const c_uchar,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut c_uchar,
    ) -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPError;

impl std::fmt::Display for WebPError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("WebP format error")
    }
}

impl std::error::Error for WebPError {}

pub fn decode_buf(data: &[u8]) -> Result<DecodeOutput, WebPError> {
    let mut w: i32 = 0;
    let mut h: i32 = 0;

    let result = unsafe { decode(data.as_ptr(), data.len(), &mut w, &mut h) };
    if result.is_null() {
        return Err(WebPError);
    }
    Ok(DecodeOutput {
        buf: unsafe { std::slice::from_raw_parts(result, w as usize * h as usize * 4) }.to_vec(),
        dimensions: (w as u32, h as u32),
    })
}

pub(crate) fn decode_webp<P: AsRef<Path>>(path: P) -> Result<DecodeOutput, ImageDiffError> {
    let mut file = File::open(path.as_ref())?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    decode_buf(&buf).map_err(|_| {
        ImageDiffError::DecodeError(path.as_ref().to_str().expect("should convert").to_string())
    })
}

fn encode_buf(rgba: &[u8], width: u32, height: u32) -> Result<EncodeOutput, WebPError> {
    // For now reserve rgba size.
    let mut output: Vec<u8> = Vec::with_capacity(rgba.len());
    let ptr = output.as_mut_ptr();

    let result = unsafe {
        encode(
            rgba.as_ptr(),
            width as i32,
            height as i32,
            (width * 4) as i32,
            ptr,
        )
    };
    if result == 0 {
        return Err(WebPError);
    }
    let res = unsafe { std::slice::from_raw_parts(ptr, result) }.to_vec();
    Ok(EncodeOutput { buf: res })
}

pub(crate) fn encode_webp<P: AsRef<Path>>(
    path: P,
    rgba: &[u8],
    width: u32,
    height: u32,
) -> Result<(), ImageDiffError> {
    let result = encode_buf(rgba, width, height).unwrap();
    let mut file = File::create("./test.webp").unwrap();
    file.write_all(&result.buf)?;
    file.flush()?;
    dbg!("asdad");
    Ok(())
}
