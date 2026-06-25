use ab_glyph::{FontRef, PxScale};
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};

pub fn get_max_font_size(text: &str, font: &FontRef, max_width: u32, max_height: u32) -> f32 {
    let mut font_size = 1.0;
    loop {
        let scale = PxScale::from(font_size);
        let (width, height) = text_size(scale, font, text);
        if width > max_width || height > max_height {
            return font_size - 1.0;
        }
        font_size += 1.0;
        if font_size > 500.0 { return font_size; }
    }
}

pub fn draw_centered_text(
    img: &mut RgbaImage,
    text: &str,
    font: &FontRef,
    image_width: u32,
    image_height: u32,
) {
    let first_pixel = img.get_pixel(0, 0);
    let text_colour = Rgba([
        (255 - (first_pixel[0] as f32 * 1.2).min(255.0) as u8),
        (255 - (first_pixel[1] as f32 * 1.2).min(255.0) as u8),
        (255 - (first_pixel[2] as f32 * 1.2).min(255.0) as u8),
        255,
    ]);

    let horizontal_padding = image_width as f32 * 0.1;
    let usable_width = image_width as f32 - 2.0 * horizontal_padding;
    let vertical_padding = image_height as f32 * 0.1;
    let usable_height = image_height as f32 - 2.0 * vertical_padding;

    let font_size = get_max_font_size(text, font, usable_width as u32, usable_height as u32);
    let scale = PxScale::from(font_size);

    let (text_width, text_height) = text_size(scale, font, text);
    let position_x = (image_width as i32 - text_width as i32) / 2;
    let position_y = (image_height as i32 - text_height as i32) / 2;

    let position_x = position_x.max(horizontal_padding as i32);
    let position_y = position_y.max(vertical_padding as i32);

    draw_text_mut(img, text_colour, position_x, position_y, scale, font, text);
}

pub fn apply_noise(img: &mut RgbaImage) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for pixel in img.pixels_mut() {
        let n: f32 = rng.gen_range(-25.0..25.0);
        pixel[0] = (pixel[0] as f32 * 0.9 + (pixel[0] as f32 + n).clamp(0.0, 255.0) * 0.1) as u8;
        pixel[1] = (pixel[1] as f32 * 0.9 + (pixel[1] as f32 + n).clamp(0.0, 255.0) * 0.1) as u8;
        pixel[2] = (pixel[2] as f32 * 0.9 + (pixel[2] as f32 + n).clamp(0.0, 255.0) * 0.1) as u8;
    }
}
