# Noise Generator

A simple noise generator written in Rust.

## How to build

- Set up Rust toolchain.
- Run `cargo build -r`.

## Configuration

### Mode

- Define noise generation mode.
    - `"BlackAndWhiteOnly"` will only uses black and white. **[DEFAULT]**
    - `"Grayscale"` will additionally uses grayscale colors.
    - `"Rainbow"` will uses all colors from RGBA-U8 pallette.
    - `"Red"`, `"Green"`, `"Blue"` will only uses the respectives colors and their shades.

### Format

- Define output image format. Possible values:
    - `"png"` **[DEFAULT]**
    - `"jpeg"`
    - `"webp"`
    - `"bmp"`

### Resolution

- Define output image resolution.
    - `"width"` defines output's width.
    - `"height"` defines output's height.
