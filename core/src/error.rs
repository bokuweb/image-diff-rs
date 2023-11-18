use thiserror::Error;

#[derive(Error, Debug)]

pub enum ImageDiffError {
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error("specified input file extension is not supported. this library support `*.jpg`, `*.jpeg`, `*.gif`, `*.webp`, `*.png`, `*.tiff`, `*.bmp` files. but got {0}")]
    InputExtension(String),
    #[error("specified output file extension is not supported. this library support `*.webp`, `*.png` files. but got {0}")]
    OutputExtension(String),
    #[error("it is not able to decode {0}.")]
    Decode(String),
    #[error("it is not able to encode {0}.")]
    Encode(String),
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error("unknown diff error")]
    Unknown,
}

impl From<pixelmatch::PixelmatchError> for ImageDiffError {
    fn from(_error: pixelmatch::PixelmatchError) -> Self {
        ImageDiffError::Unknown
    }
}
