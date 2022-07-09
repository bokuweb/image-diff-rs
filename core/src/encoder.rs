use std::path::Path;

use image::{GenericImageView, ImageBuffer};

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EncodeOutput {
    pub buf: Vec<u8>,
}

pub fn encode<P: AsRef<Path>>(
    path: P,
    rgba: &[u8],
    width: u32,
    height: u32,
) -> Result<(), ImageDiffError> {
    dbg!("--");
    let mut img = ImageBuffer::from_fn(width, height, |x, y| {
        let s = (y * width * 4 + x * 4) as usize;
        let r = rgba[s];
        let g = rgba[s + 1];
        let b = rgba[s + 2];
        let a = rgba[s + 3];
        image::Rgba([r, g, b, a])
    });
    // let dimg = image::load_from_memory(rgba).expect("Should load image from memory.");
    let size = img.dimensions();
    // let mut image = std::io::Cursor::new(vec![]);
    dbg!(size);
    let dimg = image::DynamicImage::from(img);
    dimg.save("./aaaa.png").unwrap();
    // For now only png supported
    // let p = path.as_ref();
    // let ext = p.extension().ok_or_else(|| {
    //     ImageDiffError::ExtensionError(p.to_str().expect("should convert").to_string())
    // })?;
    //
    // match ext.to_str() {
    //     Some("webp") => Ok(decode_webp(path)?),
    //     Some(_) => {
    //         todo!(),
    //     }
    //     None => Err(ImageDiffError::ExtensionError(
    //         "Failed to detect extension.".to_owned(),
    //     )),
    // }
    Ok(())
}
