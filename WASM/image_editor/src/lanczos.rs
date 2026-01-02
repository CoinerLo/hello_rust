use crate::image::Image;

pub fn resize(src: &Image, dst_width: usize, dst_height: usize, r: usize) -> Image {
    assert!(dst_width > 0 && dst_height > 0, "Размеры целевого изображения должны быть положительными");
    assert!(src.width > 0 && src.height > 0, "Размерв исходного изображения должны быть положительнми");

    // Поддержка ядра от -(R-1) до + (R-1)
    let support = (r as f32) - 1.0;
    let kernel_size = 2 * r - 1;

    // Создаём имметричное ядро Lanczos
    let mut kernel = vec![0.0f32; kernel_size];

    for i in 0..kernel_size {
        let x = i as isize - (r as isize - 1);
        kernel[i] = lanczos(x, r);
    }

    // Промежуточный буфер: dst_width × src_height
    let mut dst_hor = Image::new(dst_width, src.height);

    // Горизонтальный проход
    resize_horizontal(src, &mut dst_hor, &kernel, support);

    // Финальный буфер: dst_width × dst_height
    let mut dst = Image::new(dst_width, dst_height);

    // Вертикальный проход
    resize_vertical(&dst_hor, &mut dst, &kernel, support);

    dst

}

fn lanczos(x: isize, radius: usize) -> f32 {
    if x == 0 {
        return 1.0;
    }
    let abs_x = x.abs() as f32;
    if abs_x > radius as f32 {
        return 0.0;
    }
    let pi_x = std::f32::consts::PI * abs_x;
    let r = radius as f32;

    r * pi_x.sin() * (pi_x / r).sin() / (pi_x * pi_x)
}

fn resize_horizontal(src: &Image, dst: &mut Image, kernel: &[f32], support: f32) {
    let src_width = src.width;
    let src_height = src.height;
    let dst_width = dst.width;
    let scale_x = src_width as f32 / dst_width as f32;

    for y in 0..src_height {
        let src_offset_base = y * src_width * 4;
        let dst_offset_base = y * dst_width * 4;
        for dst_x in 0..dst_width {
            let src_x_center = (dst_x as f32 + 0.5) * scale_x;
            let start = (src_x_center - support).floor() as isize;
            let end = (src_x_center + support).floor() as isize;

            let mut sum_r = 0.;
            let mut sum_g = 0.;
            let mut sum_b = 0.;
            let mut sum_a = 0.;
            let mut sum_w = 0.;

            for src_x_i in start..=end {
                let src_x = src_x_i.clamp(0, src_width as isize - 1) as usize;
                let rel_i = (src_x_i - start) as usize;
                if rel_i >= kernel.len() {
                    continue;
                }
                let weight = kernel[rel_i];
                if weight.abs() < 1e-5 {
                    continue;
                }
                let idx = src_offset_base + src_x * 4;

                sum_r += src.data[idx] as f32 * weight;
                sum_g += src.data[idx + 1] as f32 * weight;
                sum_b += src.data[idx + 2] as f32 * weight;
                sum_a += src.data[idx + 3] as f32 * weight;

                sum_w += weight;
            }
            let out_idx = dst_offset_base + dst_x * 4;
            if sum_w > 1e-5 {
                let inv_w = 1.0f32 / sum_w;
                dst.data[out_idx] = ((sum_r *inv_w) + 0.5).clamp(0.0, 255.0) as u8;
                dst.data[out_idx + 1] = ((sum_g *inv_w) + 0.5).clamp(0.0, 255.0) as u8;
                dst.data[out_idx + 2] = ((sum_b *inv_w) + 0.5).clamp(0.0, 255.0) as u8;
                dst.data[out_idx + 3] = ((sum_a *inv_w) + 0.5).clamp(0.0, 255.0) as u8;
            } else {

            }
        }
    }

}

fn resize_vertical(src: &Image, dst: &mut Image, kernel: &[f32], support: f32) {

}
