use thiserror::Error;

#[derive(Error, Debug)]

pub enum ImageDiffError {
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error("specified input file extension is not supported. this library support `*.jpg`, `*.jpeg`, `*.gif`, `*.webp`, `*.png`, `*.tiff`, `*.bmp` files. but got {0}")]
    InputExtension(String),
    #[error("it is not able to decode {0}.")]
    Decode(String),
    #[error("it is not able to encode {0}.")]
    Encode(String),
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error("input buf length error. please input same length images.")]
    ImageLength,
}

impl From<pixelmatch::PixelmatchError> for ImageDiffError {
    fn from(error: pixelmatch::PixelmatchError) -> Self {
        match error {
            pixelmatch::PixelmatchError::ImageLengthError => Self::ImageLength,
            _ => unreachable!(),
        }
    }
}
