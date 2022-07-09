use std::path::Path;

mod compare;
mod decoder;
mod encoder;
mod error;
mod types;
mod webp;

pub use compare::*;
pub use decoder::*;
pub use encoder::*;
pub use error::*;
pub use types::*;
pub use webp::*;

pub fn diff<P: AsRef<Path>>(input: &CompareInput<P>) -> Result<DiffOutput, ImageDiffError> {
    let decoded1 = decode(input.actual_filename.as_ref())?;
    let decoded2 = decode(input.expected_filename.as_ref())?;
    let p = "";
    let result = compare(input)?;
    dbg!(&result.width, &result.height, result.diff_count);
    encode(
        input.diff_filename.as_ref().unwrap(),
        &result.diff_image,
        result.width,
        result.height,
    )
    .unwrap();
    Ok(result)
}
