use image_diff_rs::DiffOption;

wit_bindgen::generate!({
 world: "image-diff",
 exports: {
     world: ImageDiff,
 }
});

struct ImageDiff;

impl Guest for ImageDiff {
    fn diff(
        imga: Vec<u8>,
        imgb: Vec<u8>,
        opts: bokuweb::image_diff::types::Opts,
    ) -> Result<bokuweb::image_diff::types::Output, bokuweb::image_diff::types::Error> {
        let _res = image_diff_rs::diff(
            imga,
            imgb,
            &DiffOption {
                threshold: opts.threshold,
                include_anti_alias: opts.include_anti_alias,
            },
        );
        todo!();
    }
}
