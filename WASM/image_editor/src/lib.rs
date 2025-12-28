mod image;
mod lanczos;

use wasm_bindgen::prelude::*;

use image::Image;

#[wasm_bindgen]
pub fn wasm_resize(
    data: &[u8],
    src_width: usize,
    src_height: usize,
    dst_width: usize,
    dst_height: usize,
    r: usize
) -> Vec<u8> {
    let src_image = Image::from(data, src_width, src_height);

    lanczos::resize(&src_image, dst_width, dst_height, r).data
}
