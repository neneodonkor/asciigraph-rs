# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] – [CURRENTLY UNDER DEVELOPMENT]

### Added
- Zero-line highlighting via `ZeroLine` — an opt-in horizontal reference line
  drawn at Y = 0.0 across the data area when the plotted range straddles zero.
  Configurable with a custom character and ANSI color. Enabled through the
  `Config::zero_line()` builder method.

- Threshold lines via `Threshold` — opt-in horizontal reference lines drawn
  at user-specified Y values using a dashed character (`╌`). Each threshold
  carries its own value, color, and character independently, making it easy
  to distinguish warning and critical levels. Multiple thresholds can be added
  by chaining `Config::threshold()` calls. Thresholds outside the visible Y
  range are silently ignored. Series arc characters always render on top.

- Added `examples/zero_line.rs` demonstrating zero-line highlighting with a
  dataset that straddles zero, including a colored variant using `ZeroLine::with_color()`.
- Added `examples/threshold.rs` demonstrating two threshold lines at Y = 80.0
  and Y = 90.0 rendered in different ANSI colors over a 20-point dataset.

- Moving average overlay via `Config::moving_average(window)` — computes a
  sliding window average over the first series and renders it as an additional
  series on the same graph. NaN values are excluded from the window average.
  A window of 0 or 1 has no effect. Pairs naturally with `series_colors` to
  visually distinguish the smoothed series from the raw data.

- Added `examples/moving_average.rs` demonstrating a 5-point moving average
  overlay rendered in yellow over a noisy 20-point dataset.

- Improved documentation comments across `utils.rs` — all utility functions
  now have doc comments with descriptions, parameter notes, and panic
  conditions where applicable.

### Changed
- Refactored `asciigraph.rs` into a module directory (`src/asciigraph/`) with
  a dedicated `features/` subdirectory. Self-contained rendering features are
  now in separate files:
    - `features/zero_line.rs` — `render_zero_line`
    - `features/threshold.rs` — `render_thresholds`
    - `features/x_axis.rs` — `add_x_axis`

- Improved documentation comments across `options.rs` and `asciigraph.rs` —
  all public types, fields, and methods now have doc comments with descriptions,
  parameter notes, and usage examples where appropriate.

- Auto tick count for the X-axis — when `x_axis_tick_count` is not set, the
  library now automatically calculates a sensible number of ticks based on
  the available graph width and the estimated label width, instead of
  defaulting to a hardcoded value of 5. Explicitly setting `x_axis_tick_count`
  still overrides the calculation.

- Added `examples/auto_tick_count.rs` demonstrating automatic tick calculation
  on a 20-point dataset with no explicit tick count configured.

- Refactored `plot_many` into focused, single-responsibility helper functions.
  The monolithic function is now decomposed into:
    - `normalize_config` — applies default values to offset and line ending
    - `prepare_data` — deep-copies, pads, and interpolates input series
    - `calculate_bounds` — derives min/max, ratio, and scaled integer bounds,
      returned as a named `Bounds` struct
    - `init_grid` — allocates the blank 2-D `Cell` grid
    - `calculate_precision` — computes Y-axis label decimal places
    - `calculate_y_axis_magnitudes` — computes the real-valued magnitude and
      maximum label width for each grid row
    - `render_y_axis` — writes Y-axis labels and tick characters into the grid
    - `render_series` — draws arc and line characters for each data series
    - `render_zero_line` — draws the zero line into the grid before series rendering
    - `join_rows` — flattens the 2-D grid into a single ANSI-colored string
    - `render_caption` — appends the caption below the plot body

[0.1.4]: https://github.com/neneodonkor/asciigraph-rs/compare/v0.1.3...v0.1.4

## [0.1.3] - 2026-04-29

### Changed
- Refactored `plot_many` in `asciigraph.rs` to be more idiomatic Rust
  based on community feedback. Specific improvements include:
    - Replaced manual index loop with `slice::windows(2)` for more
      expressive iteration over consecutive data point pairs
    - Replaced manual trailing space search with `trim_end_matches`
      for cleaner row string building
    - Replaced NaN padding loop with `Vec::resize()` for cleaner
      and more efficient buffer extension
    - Used `if` as an expression for magnitude calculation instead
      of imperative assignment
    - Used iterator chain with `.map(Vec::len).max().unwrap_or(0)`
      for finding maximum series length

[0.1.3]: https://github.com/neneodonkor/asciigraph-rs/compare/v0.1.2...v0.1.3

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