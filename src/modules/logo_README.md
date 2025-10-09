# Netrunner Logo Widget

A cyberpunk-styled logo widget for the Netrunner CLI application, inspired by the Ratatui logo implementation.

## Features

- **Programmatic Drawing**: The logo is drawn using geometric shapes and lines, not pre-rendered ASCII art
- **Cyberpunk Aesthetic**: Uses neon cyan, magenta, yellow, and green colors for a futuristic look
- **Multiple Sizes**: Three size variants (Tiny, Small, Medium) for different use cases
- **Pure Rust**: Built with Ratatui's rendering primitives

## Usage

```rust
use netrunner_cli::modules::{NetrunnerLogo, NetrunnerLogoSize};
use ratatui::widgets::Widget;

// Create a medium-sized logo (default)
let logo = NetrunnerLogo::new(NetrunnerLogoSize::Medium);
frame.render_widget(logo, area);

// Or use a different size
let small_logo = NetrunnerLogo::new(NetrunnerLogoSize::Small);
let tiny_logo = NetrunnerLogo::new(NetrunnerLogoSize::Tiny);
```

## Size Variants

### Medium (Default)
- Height: 7 lines
- Width: ~70 characters
- Best for: Main splash screens and headers

### Small
- Height: 5 lines
- Width: ~50 characters
- Best for: Compact displays and sidebars

### Tiny
- Height: 3 lines
- Width: ~35 characters
- Best for: Inline display and status bars

## Color Palette

The logo uses a cyberpunk-inspired color scheme:

- **Cyan** (RGB: 0, 255, 255): Primary color for main letters
- **Cyan Bright** (RGB: 100, 255, 255): Highlights and accents
- **Cyan Dim** (RGB: 0, 200, 200): Bottom accents
- **Magenta** (RGB: 255, 0, 255): Accent letters and glitch effects
- **Yellow** (RGB: 255, 255, 0): Contrast letters
- **Green Neon** (RGB: 0, 255, 150): Accent letters

## Example

Run the included example to see all size variants:

```bash
# Medium size (default)
cargo run --example logo_demo

# Small size
cargo run --example logo_demo small

# Tiny size
cargo run --example logo_demo tiny
```

## Implementation Details

The logo is rendered using Ratatui's `Widget` trait and draws each letter programmatically using:

- `draw_horizontal_line()`: For horizontal bars and accents
- `draw_vertical_line()`: For vertical strokes
- `draw_diagonal_line()`: For diagonal connections
- `draw_cell()`: For individual character placement

This approach allows for:
- Consistent rendering across different terminal emulators
- Easy color customization
- Efficient buffer manipulation
- No external font or image dependencies

## Design Philosophy

The Netrunner logo embodies the cyberpunk aesthetic with:

1. **Geometric Precision**: Clean lines and shapes create a technical, digital feel
2. **Neon Colors**: Bright cyan and magenta evoke neon-lit dystopian cityscapes
3. **Glitch Elements**: Strategic use of Unicode block characters (▀▄█▓▒░) add texture
4. **High Contrast**: Sharp color transitions create visual impact

## Future Enhancements

Potential improvements for future versions:

- [ ] Animated glitch effects
- [ ] Customizable color schemes
- [ ] Additional size variants (Large, Extra Large)
- [ ] Animation support (fade-in, scan lines)
- [ ] Theme integration with terminal color schemes