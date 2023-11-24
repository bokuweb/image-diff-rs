pub(crate) fn expand(
    original_image: Vec<u8>,
    original_dimensions: (u32, u32),
    expand_width: u32,
    expand_height: u32,
) -> Vec<u8> {
    let orig_width = original_dimensions.0;
    let orig_height = original_dimensions.1;

    if orig_width == expand_width && orig_height == expand_height {
        return original_image;
    }

    let mut new_data = vec![0; (expand_width * expand_height * 4) as usize];

    for j in 0..expand_height {
        if j < orig_height {
            for i in 0..expand_width {
                let idx = (j * expand_width + i) * 4;
                if i < orig_width {
                    let orig_idx = (j * orig_width + i) * 4;
                    new_data[idx as usize] = original_image[orig_idx as usize];
                    new_data[(idx + 1) as usize] = original_image[(orig_idx + 1) as usize];
                    new_data[(idx + 2) as usize] = original_image[(orig_idx + 2) as usize];
                    new_data[(idx + 3) as usize] = original_image[(orig_idx + 3) as usize];
                }
            }
        }
    }
    new_data
}
