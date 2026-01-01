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

}

fn resize_vertical(src: &Image, dst: &mut Image, kernel: &[f32], support: f32) {
    
}
