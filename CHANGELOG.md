# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2026-04-28

### Changed
- Improved `color.rs` with more idiomatic Rust patterns. `AnsiColor` now implements the standard `FromStr` and `TryFrom<&str>` traits, meaning colors can be parsed using Rust's built-in `.parse::<AnsiColor>()` syntax in addition to the existing `get_ansi_color` method.
- Added string sanitization to color name lookup — color names are now case-insensitive and accept underscore separators, so `"AliceBlue"`, `"aliceblue"`, and `"alice_blue"` all resolve correctly.
- Added `#[repr(transparent)]` to `AnsiColor` to guarantee zero-cost wrapping over its inner `u8` value.
- Updated crate description in `Cargo.toml` for clarity.

## [0.1.1] - 2026-04-27

### Changed
- Updated crate description in `Cargo.toml`.

## [0.1.0] - 2026-04-26

### Added
- Initial release — a complete Rust port of [guptarohit/asciigraph](https://github.com/guptarohit/asciigraph) (Go v0.9.0).
- `plot` function for rendering a single data series as an ASCII line graph.
- `plot_many` function for rendering multiple data series on the same graph.
- `Config` builder pattern for configuring graph options including height, width, offset, precision, caption, lower bound, upper bound, line ending, and series character sets.
- `AnsiColor` type with over 140 named color constants covering the full W3C SVG color keyword list.
- ANSI color support for series lines, Y-axis, axis labels, and captions.
- Series legends with centered layout and colored legend markers.
- X-axis support with configurable tick count and tick label formatting via `x_axis_range` and `x_axis_tick_count`.
- Custom Y-axis value formatter via `y_axis_value_formatter` — accepts any closure of the form `Fn(f64) -> String`.
- Custom X-axis value formatter via `x_axis_value_formatter`.
- Custom character sets for plot lines via `CharSet` and `create_char_set`.
- NaN gap handling with proper start caps (`╶`) and end caps (`╴`) at series boundaries.
- Configurable line endings including CRLF support for Windows terminals.
- Full CLI binary (`asciigraph`) with argument parsing via `clap`, supporting all library options as command line flags.
- Realtime streaming mode (`-r` flag) with configurable FPS for continuously updating graphs from stdin.
- Built-in data generator (`datagen`) for demonstrating realtime mode on Windows, where pipe buffering prevents direct shell pipelines from working correctly.
- Full test suite ported from the Go original with 67 passing tests.
- Five example programs covering basic graphs, multiple series, sine curves, rainbow-colored graphs, and X-axis usage.

[0.1.2]: https://github.com/neneodonkor/asciigraph-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/neneodonkor/asciigraph-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/neneodonkor/asciigraph-rs/releases/tag/v0.1.0