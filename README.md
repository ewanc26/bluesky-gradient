# Bluesky Gradient Image Generator

[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

Generates gradient profile pictures and banners for Bluesky, one per hour.

## Features

- 24 unique images, one for each hour
- Draws your name (or any text) with auto-adjusted contrast colours
- Subtle noise grain
- Profile pics (400x400), banners (1500x500), or custom sizes

## Requirements

- Rust 1.85+

## Install

```bash
git clone https://github.com/ewanc26/bluesky-gradient.git
cd bluesky-gradient
cargo build --release
```

Config goes in `./config/generation.json` (colours + display name).

## Usage

```bash
# Profile pics
cargo run --release -- --profile

# Banners
cargo run --release -- --banner

# Custom size
cargo run --release -- --custom --width 800 --height 600
```

Images are saved to `src/profile_pics` or `src/banners`.

## Files

- `src/main.rs` — Generation logic
- `config/generation.json` — Colour and name config
- `config/fonts/` — Font file directory

## Licence

MIT

## Support
If you find this project useful, consider supporting its development:
[![Ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/ewancroft)
[![GitHub Sponsors](https://img.shields.io/badge/GitHub%20Sponsors-30363D?style=for-the-badge&logo=github&logoColor=white)](https://github.com/sponsors/ewanc26)
