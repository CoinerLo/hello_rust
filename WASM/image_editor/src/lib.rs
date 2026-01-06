mod image;
mod lanczos;

use wasm_bindgen::{JsCast, Clamped};
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, ImageData};
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

fn console_log(s: &str) {
    web_sys::console::log_1(&JsValue::from_str(s));
}

#[wasm_bindgen]
pub fn wasm_auto_resize() -> Result<(), JsValue> {
    console_log("WASM загружен, начинаем обработку canvas...");
    let window = web_sys::window().expect("No global window exist");
    let document = window.document().expect("should have a document on window");

    let canvas = document
        .get_element_by_id("editor")
        .expect(r#"no canvas with id "editor""#)
        .dyn_into::<HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Загружаем изображение (предполагаем, что оно уже нарисовано на canvas)

    let width = canvas.width() as usize;
    let height = canvas.height() as usize;

    if width == 0 || height == 0 {
        console_log("The canvas is empty");
        return Ok(());
    }

    // Получаем данные пикселей
    let image_data = context.get_image_data(0.0, 0.0, width as f64, height as f64)?;
    let src_data = image_data.data();

    let src_image = Image {
        data: src_data.to_vec(),
        width,
        height,
    };

    let new_width = width * 2;
    let new_height = height * 2;

    console_log(&format!("Ресайз: {}x{} -> {}x{}", width, height, new_width, new_height));

    let resaized_image = lanczos::resize(&src_image, new_width, new_height, 3);

    let new_image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&resaized_image.data),
        new_width as u32,
        new_height as u32
    )?;

    canvas.set_width(new_width as u32);
    canvas.set_height(new_height as u32);

    context.put_image_data(&new_image_data, 0.0, 0.0)?;

    console_log("Ресайз завершен и отображен");

    Ok(())
}
