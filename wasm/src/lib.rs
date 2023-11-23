use image_diff_rs::{DiffOption, DiffOutput, ImageDiffError};

wit_bindgen::generate!({
 world: "image-diff",
 exports: {
     world: ImageDiff,
 }
});

struct ImageDiff;

impl From<ImageDiffError> for bokuweb::image_diff::types::Error {
    fn from(value: ImageDiffError) -> Self {
        match value {
            ImageDiffError::Decode(s) => bokuweb::image_diff::types::Error::Decode(s),
            ImageDiffError::Encode(s) => bokuweb::image_diff::types::Error::Encode(s),
            ImageDiffError::InputExtension(_)
            | ImageDiffError::Image(_)
            | ImageDiffError::File(_) => {
                unreachable!()
            }
        }
    }
}

impl From<DiffOutput> for bokuweb::image_diff::types::Output {
    fn from(value: DiffOutput) -> Self {
        Self {
            diff_count: value.diff_count as u32,
            diff_image: value.diff_image,
            width: value.width,
            height: value.height,
        }
    }
}

impl Guest for ImageDiff {
    fn diff(
        imga: Vec<u8>,
        imgb: Vec<u8>,
        opts: bokuweb::image_diff::types::Opts,
    ) -> Result<bokuweb::image_diff::types::Output, bokuweb::image_diff::types::Error> {
        Ok(image_diff_rs::diff(
            imga,
            imgb,
            &DiffOption {
                threshold: opts.threshold,
                include_anti_alias: opts.include_anti_alias,
            },
        )?
        .into())
    }
}
