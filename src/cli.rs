use clap::Parser;

/// CLI arguments for the bluesky-gradient image generator.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Generate profile image (400x400)
    #[arg(short, long)]
    pub profile: bool,

    /// Generate banner image (1500x500)
    #[arg(short, long)]
    pub banner: bool,

    /// Generate custom image with custom width and height
    #[arg(short, long)]
    pub custom: bool,

    /// Custom image width (required with -c)
    #[arg(short, long)]
    pub width: Option<u32>,

    /// Custom image height (required with -c)
    #[arg(short, long, name = "HEIGHT")]
    pub custom_height: Option<u32>,
}

/// Resolved dimensions and output path for the current run.
pub struct ImageParams {
    pub width: u32,
    pub height: u32,
    pub output_folder_base: String,
}

impl Args {
    /// Translate the user-facing flag into concrete image dimensions.
    ///
    /// Exactly one of --profile, --banner, or --custom must be set; --custom
    /// additionally requires --width and --height.
    pub fn image_params(&self) -> anyhow::Result<ImageParams> {
        if self.profile {
            Ok(ImageParams { width: 400, height: 400, output_folder_base: "src/profile_pics".into() })
        } else if self.banner {
            Ok(ImageParams { width: 1500, height: 500, output_folder_base: "src/banners".into() })
        } else if self.custom {
            let w = self.width.context("Width required for custom mode")?;
            let h = self.custom_height.context("Height required for custom mode")?;
            Ok(ImageParams { width: w, height: h, output_folder_base: format!("src/custom_{}x{}", w, h) })
        } else {
            anyhow::bail!("Specify either -p, -b, or -c");
        }
    }
}
