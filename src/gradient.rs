use image::{ImageBuffer, Rgba, RgbaImage};
use palette::{LinSrgb, Mix};
use std::collections::HashMap;

use crate::config::Config;

pub fn interpolate_colour(hour: u32, sky_colours: &HashMap<String, Vec<u8>>) -> Rgba<u8> {
    let mut hours: Vec<u32> = sky_colours.keys().map(|k| k.parse().unwrap()).collect();
    hours.sort_unstable();

    for i in 0..hours.len() - 1 {
        if hours[i] <= hour && hour <= hours[i + 1] {
            let t = (hour - hours[i]) as f32 / (hours[i + 1] - hours[i]) as f32;
            let c1 = &sky_colours[&hours[i].to_string()];
            let c2 = &sky_colours[&hours[i + 1].to_string()];

            let color1 = LinSrgb::new(c1[0] as f32 / 255.0, c1[1] as f32 / 255.0, c1[2] as f32 / 255.0);
            let color2 = LinSrgb::new(c2[0] as f32 / 255.0, c2[1] as f32 / 255.0, c2[2] as f32 / 255.0);
            let mixed = color1.mix(color2, t);

            return Rgba([
                (mixed.red * 255.0) as u8,
                (mixed.green * 255.0) as u8,
                (mixed.blue * 255.0) as u8,
                255,
            ]);
        }
    }

    let c = &sky_colours[&hour.to_string()];
    Rgba([c[0], c[1], c[2], 255])
}

pub fn create_gradient(hour: u32, width: u32, height: u32, sky_colours: &HashMap<String, Vec<u8>>) -> RgbaImage {
    let colour = interpolate_colour(hour, sky_colours);
    let avg = (colour[0] as f32 + colour[1] as f32 + colour[2] as f32) / 3.0;

    let fade_ratio = 0.1 + (0.5 - 0.1) * (1.0 - avg / 255.0);
    let gradient_height = (height as f32 * fade_ratio) as u32;
    let main_height = height - gradient_height;

    let mut img = ImageBuffer::new(width, height);
    let mono_val = avg as u8;
    let mono_colour = Rgba([mono_val, mono_val, mono_val, 255]);

    for y in 0..main_height {
        for x in 0..width {
            img.put_pixel(x, y, colour);
        }
    }

    for y in 0..gradient_height {
        let t = y as f32 / gradient_height as f32;
        let color1 = LinSrgb::new(colour[0] as f32 / 255.0, colour[1] as f32 / 255.0, colour[2] as f32 / 255.0);
        let color2 = LinSrgb::new(mono_colour[0] as f32 / 255.0, mono_colour[1] as f32 / 255.0, mono_colour[2] as f32 / 255.0);
        let mixed = color1.mix(color2, t);
        let mixed_rgba = Rgba([
            (mixed.red * 255.0) as u8,
            (mixed.green * 255.0) as u8,
            (mixed.blue * 255.0) as u8,
            255,
        ]);

        for x in 0..width {
            img.put_pixel(x, main_height + y, mixed_rgba);
        }
    }

    img
}
