use thiserror::Error;

#[derive(Error, Debug)]

pub enum ImageDiffError {
    #[error(transparent)]
    FileOpen(#[from] std::io::Error),
    #[error("specified input file extension is not supported. this library support `*.jpg`, `*.jpeg`, `*.gif`, `*.webp`, `*.png`, `*.tiff`, `*.bmp` files. but got {0}")]
    InputExtensionError(String),
    #[error("specified output file extension is not supported. this library support `*.webp`, `*.png` files. but got {0}")]
    OutputExtensionError(String),
    #[error("it is not able to decode {0}.")]
    DecodeError(String),
    #[error("it is not able to encode {0}.")]
    EncodeError(String),
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
