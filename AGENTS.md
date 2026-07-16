# AGENTS.md

Guidance for agents working on the Rust gradient avatar/banner generator.

## Structure and behavior

- `src/main.rs` and adjacent modules parse CLI arguments, load generation configuration, create gradients/noise, select readable text colors, and save images.
- `config/generation.json` is the user-facing palette/name schema.
- The bundled font is a runtime asset; preserve licensing, path resolution, and clean-checkout behavior.

## Invariants

- Profile output is 400x400 and banner output 1500x500 unless custom dimensions are explicitly selected.
- Validate dimensions and allocation sizes before creating image buffers; reject zero or unreasonable values.
- Maintain sufficient text/background contrast and keep text within image bounds for long or Unicode names.
- Seedable/deterministic generation should remain possible for tests even if normal output uses randomness.
- Create output directories safely and report write failures; do not silently overwrite unrelated files.
- The project is marked unmaintained, so keep changes narrowly scoped.

## Validation

Run `cargo fmt --check`, `cargo clippy --all-targets`, `cargo test`, and `cargo build --release`. Generate profile, banner, and custom images from a temporary directory; inspect dimensions, file format, text placement, contrast, missing-font behavior, malformed configuration, and boundary dimensions. Do not commit generated image batches.
