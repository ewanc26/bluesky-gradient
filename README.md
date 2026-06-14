# Bluesky Gradient Image Generator

[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

A Rust tool to generate beautiful gradient profile pictures and banners for Bluesky, with hourly variations.

## Features

- **Hourly Gradients**: Generates 24 unique images, one for each hour of the day.
- **Dynamic Text**: Adds your name (or any text) to the images with automatically adjusted contrast colors.
- **Noise Effect**: Adds a subtle noise grain for a modern look.
- **Multiple Sizes**: Supports profile pictures (400x400), banners (1500x500), and custom dimensions.

## Requirements

- Rust 1.85+ (Cargo)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/ewanc26/bluesky-gradient.git
   cd bluesky-gradient
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Configuration:
   The tool uses `./config/generation.json` to define colors and the name to display.

## Usage

Run the generator with the desired flags:

```bash
# Generate profile pictures (400x400)
cargo run --release -- --profile

# Generate banners (1500x500)
cargo run --release -- --banner

# Generate custom size
cargo run --release -- --custom --width 800 --height 600
```

Images will be saved in the `src/` directory (e.g., `src/profile_pics`, `src/banners`).

## File Structure

- `src/main.rs`: Main generation logic.
- `config/generation.json`: Color and name configuration.
- `config/fonts/`: Directory containing the font file.
- `Cargo.toml`: Rust dependencies.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## ☕ Support

If you found this useful, consider [buying me a ko-fi](https://ko-fi.com/ewancroft)!
