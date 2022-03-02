use std::fs::File;
use std::io::Read;
use std::os::raw::*;
use std::path::Path;

use super::*;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPError;

impl std::fmt::Display for WebPError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("WebP format error")
    }
}

impl std::error::Error for WebPError {}

fn decode_buf(data: &[u8]) -> Result<DecodeOutput, WebPError> {
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
