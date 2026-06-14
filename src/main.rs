use ab_glyph::{FontRef, PxScale};
use anyhow::{Context, Result};
use clap::Parser;
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use palette::{LinSrgb, Mix};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Generate profile image (400x400)
    #[arg(short, long)]
    profile: bool,

    /// Generate banner image (1500x500)
    #[arg(short, long)]
    banner: bool,

    /// Generate custom image with custom width and height
    #[arg(short, long)]
    custom: bool,

    /// Custom image width (required with -c)
    #[arg(short, long)]
    width: Option<u32>,

    /// Custom image height (required with -c)
    #[arg(short, long, name = "HEIGHT")]
    custom_height: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    sky_colours: HashMap<String, Vec<u8>>,
    name: String,
}

fn interpolate_colour(hour: u32, sky_colours: &HashMap<String, Vec<u8>>) -> Rgba<u8> {
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

fn create_gradient(hour: u32, width: u32, height: u32, sky_colours: &HashMap<String, Vec<u8>>) -> RgbaImage {
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

fn get_available_folder(base_folder: &str) -> PathBuf {
    let mut counter = 1;
    let mut folder = PathBuf::from(base_folder);
    while folder.exists() {
        folder = PathBuf::from(format!("{}_{}", base_folder, counter));
        counter += 1;
    }
    folder
}

fn get_max_font_size(text: &str, font: &FontRef, max_width: u32, max_height: u32) -> f32 {
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

fn main() -> Result<()> {
    let args = Args::parse();

    let config_data = fs::read_to_string("./config/generation.json").context("Failed to read config/generation.json")?;
    let config: Config = serde_json::from_str(&config_data)?;

    let (width, height, output_folder_base) = if args.profile {
        (400, 400, "src/profile_pics".to_string())
    } else if args.banner {
        (1500, 500, "src/banners".to_string())
    } else if args.custom {
        let w = args.width.context("Width required for custom mode")?;
        let h = args.custom_height.context("Height required for custom mode")?;
        (w, h, format!("src/custom_{}x{}", w, h))
    } else {
        anyhow::bail!("Specify either -p, -b, or -c");
    };

    let font_path = "./config/fonts/madecarvingsoft.ttf";
    let font_data = fs::read(font_path).context("Failed to read font file")?;
    let font = FontRef::try_from_slice(&font_data).context("Failed to load font")?;

    let output_folder = get_available_folder(&output_folder_base);
    fs::create_dir_all(&output_folder)?;

    for hour in 0..24 {
        let image_path = output_folder.join(format!("{:02}.png", hour));
        if image_path.exists() { continue; }

        let mut img = create_gradient(hour, width, height, &config.sky_colours);
        
        let first_pixel = img.get_pixel(0, 0);
        let text_colour = Rgba([
            (255 - (first_pixel[0] as f32 * 1.2).min(255.0) as u8),
            (255 - (first_pixel[1] as f32 * 1.2).min(255.0) as u8),
            (255 - (first_pixel[2] as f32 * 1.2).min(255.0) as u8),
            255,
        ]);

        let horizontal_padding = width as f32 * 0.1;
        let usable_width = width as f32 - 2.0 * horizontal_padding;
        let vertical_padding = height as f32 * 0.1;
        let usable_height = height as f32 - 2.0 * vertical_padding;

        let font_size = get_max_font_size(&config.name, &font, usable_width as u32, usable_height as u32);
        let scale = PxScale::from(font_size);
        
        let (text_width, text_height) = text_size(scale, &font, &config.name);
        let position_x = (width as i32 - text_width as i32) / 2;
        let position_y = (height as i32 - text_height as i32) / 2;

        let position_x = position_x.max(horizontal_padding as i32);
        let position_y = position_y.max(vertical_padding as i32);

        draw_text_mut(&mut img, text_colour, position_x, position_y, scale, &font, &config.name);

        // Noise
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for pixel in img.pixels_mut() {
            let n: f32 = rng.gen_range(-25.0..25.0);
            // Blend 0.1
            pixel[0] = (pixel[0] as f32 * 0.9 + (pixel[0] as f32 + n).clamp(0.0, 255.0) * 0.1) as u8;
            pixel[1] = (pixel[1] as f32 * 0.9 + (pixel[1] as f32 + n).clamp(0.0, 255.0) * 0.1) as u8;
            pixel[2] = (pixel[2] as f32 * 0.9 + (pixel[2] as f32 + n).clamp(0.0, 255.0) * 0.1) as u8;
        }

        img.save(image_path)?;
    }

    println!("Images generated successfully.");
    println!("Images saved to: {:?}", output_folder);

    Ok(())
}
