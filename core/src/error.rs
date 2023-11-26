use thiserror::Error;

#[derive(Error, Debug)]

/// Enum representing potential errors that can occur during the image diffing process.
pub enum ImageDiffError {
    /// Error that occurs when decoding the input image data fail.s
    #[error("it is not able to decode {0}.")]
    Decode(String),
    /// Error that occurs when encoding the diff image data fails.
    #[error("it is not able to encode {0}.")]
    Encode(String),
}
