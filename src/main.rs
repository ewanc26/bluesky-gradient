mod cli;
mod config;
mod gradient;
mod render;

use anyhow::{Context, Result};
use cli::Args;
use clap::Parser;
use config::Config;
use render::{apply_noise, draw_centered_text};

use ab_glyph::FontRef;
use std::fs;

fn main() -> Result<()> {
    let args = Args::parse();
    let params = args.image_params()?;

    let config = Config::load("./config/generation.json")?;

    let font_data = fs::read("./config/fonts/madecarvingsoft.ttf")
        .context("Failed to read font file")?;
    let font = FontRef::try_from_slice(&font_data)
        .context("Failed to load font")?;

    let output_folder = config::get_available_folder(&params.output_folder_base);
    fs::create_dir_all(&output_folder)?;

    for hour in 0..24 {
        let image_path = output_folder.join(format!("{:02}.png", hour));
        if image_path.exists() { continue; }

        let mut img = gradient::create_gradient(hour, params.width, params.height, &config.sky_colours);
        draw_centered_text(&mut img, &config.name, &font, params.width, params.height);
        apply_noise(&mut img);

        img.save(image_path)?;
    }

    println!("Images generated successfully.");
    println!("Images saved to: {:?}", output_folder);

    Ok(())
}
