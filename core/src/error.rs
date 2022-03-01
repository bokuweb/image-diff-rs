use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageDiffError {
    /// An error occurred while interacting with the environment.
    #[error("{0:?}")]
    IoError(std::io::Error),
    #[error("file extension not detected. this library support `*.jpg`, `*.jpeg`, `*.gif`, `*.webp`, `*.png`, `*.tiff`, `*.bmp` files.")]
    ExtensionError,
    /// An error was encountered while decoding.
    ///
    /// This means that the input data did not conform to the specification of some image format,
    /// or that no format could be determined, or that it did not match format specific
    /// requirements set by the caller.
    #[error("{0:?}")]
    Decoding(image::error::DecodingError),
    /// An error was encountered in input arguments.
    ///
    /// This is a catch-all case for strictly internal operations such as scaling, conversions,
    /// etc. that involve no external format specifications.
    #[error("{0:?}")]
    Parameter(image::error::ParameterError),
    /// Completing the operation would have required more resources than allowed.
    ///
    /// Errors of this type are limits set by the user or environment, *not* inherent in a specific
    /// format or operation that was executed.
    #[error("{0:?}")]
    Limits(image::error::LimitError),
    /// An operation can not be completed by the chosen abstraction.
    ///
    /// This means that it might be possible for the operation to succeed in general but
    /// * it requires a disabled feature,
    /// * the implementation does not yet exist, or
    /// * no abstraction for a lower level could be found.
    #[error("{0:?}")]
    Unsupported(image::error::UnsupportedError),
    #[error("unknown diff error")]
    Unknown,
}

impl From<pixelmatch::PixelmatchError> for ImageDiffError {
    fn from(error: pixelmatch::PixelmatchError) -> Self {
        ImageDiffError::Unknown
    }
}

impl From<image::ImageError> for ImageDiffError {
    fn from(error: image::ImageError) -> Self {
        match error {
            image::ImageError::IoError(e) => ImageDiffError::IoError(e),
            _ => ImageDiffError::Unknown,
        }
    }
}
