// The main file that plots the graph.
mod features;
use features::zero_line::render_zero_line;
use features::threshold::render_thresholds;
use features::x_axis::add_x_axis;
use features::stat_annotations::render_stat_annotations;

use crate::legend::add_legends;
use crate::options::{CharSet, Config, DEFAULT_CHAR_SET};
use crate::utils::{calculate_height, interpolate_array, min_max_float64_slice};
use crate::{utils, AnsiColor};

// ---------------------------------------------------------------------------
// Cell
// ---------------------------------------------------------------------------

/// A single character slot in the 2-D plot grid.
///
/// The grid is initialized with default (blank, uncolored) cells before any
/// rendering pass runs. Each rendering function — `render_y_axis`,
/// `render_zero_line`, `render_thresholds`, and `render_series` — overwrites
/// specific cells with meaningful characters and colors. `join_rows` then
/// reads every cell left-to-right, top-to-bottom, and assembles the final
/// string, emitting ANSI escape codes only when the color changes.
#[derive(Clone)]
pub(crate) struct Cell {
    /// The text to render at this position. Usually a single character, but
    /// may be a multi-character string for Y-axis labels.
    text: String,
    /// The ANSI color to use when rendering this cell.
    color: AnsiColor,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            text: " ".to_string(),
            color: AnsiColor::DEFAULT,
        }
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Renders a single data series as an ASCII line graph.
///
/// This is a convenience wrapper around [`plot_many`] for the common case of
/// a single series. All configuration options available through [`Config`]
/// apply here as well.
///
/// # Example
///
/// ```rust
/// use asciigraph::{plot, Config};
///
/// let data = vec![1.0, 2.0, 3.0, 2.0, 1.0];
/// let graph = plot(&data, Config::default());
/// println!("{}", graph);
/// ```
pub fn plot(series: &[f64], config: Config) -> String {
    plot_many(&[series], config)
}

/// Renders one or more data series as an ASCII line graph.
///
/// All series share the same Y-axis scale, which is computed from the global
/// minimum and maximum across all series. Series with fewer data points than
/// the longest series are padded with `NaN` values. If a target width is set
/// via [`Config::width`], all series are interpolated to that many columns.
///
/// The rendering pipeline runs in order:
/// 1. Normalize config defaults
/// 2. Deep-copy, pad, and optionally interpolate the input data
/// 3. Calculate value bounds and grid dimensions
/// 4. Initialize the blank 2-D cell grid
/// 5. Calculate Y-axis label precision and magnitudes
/// 6. Render Y-axis labels and tick characters
/// 7. Render the zero line (if enabled)
/// 8. Render threshold lines (if any)
/// 9. Render each data series
/// 10. Flatten the grid into a string with ANSI color codes
/// 11. Append X-axis, caption, and legends (if configured)
///
/// # Example
///
/// ```rust
/// use asciigraph::{plot_many, Config, AnsiColor};
///
/// let s1 = vec![1.0, 2.0, 3.0];
/// let s2 = vec![3.0, 2.0, 1.0];
/// let graph = plot_many(
///     &[&s1, &s2],
///     Config::default().series_colors(&[AnsiColor::RED, AnsiColor::BLUE]),
/// );
/// println!("{}", graph);
/// ```
pub fn plot_many(data: &[&[f64]], config: Config) -> String {
    let config = normalize_config(config);
    let (data, len_max) = prepare_data(data, &config);
    let bounds = calculate_bounds(&data, &config);
    let width = len_max + config.offset;
    let mut plot = init_grid(bounds.rows, width);
    let precision = calculate_precision(bounds.maximum, bounds.minimum, &config);
    let (magnitudes, max_width) = calculate_y_axis_magnitudes(&bounds, precision, &config);
    let left_pad = config.offset + max_width;

    render_y_axis(&mut plot, &magnitudes, max_width, precision, &config);

    if let Some(zl) = config.zero_line {
        render_zero_line(&mut plot, &bounds, config.offset, zl);
    }

    if !config.thresholds.is_empty() {
        render_thresholds(
            &mut plot,
            &data,
            &bounds,
            config.offset,
            &config.thresholds,
            &config.series_colors,
        );
    }

    if let Some(ref sa) = config.stat_annotations {
        render_stat_annotations(&mut plot, &data, &bounds, config.offset, sa);
    }

    render_series(&mut plot, &data, &bounds, len_max, &config);

    let mut lines = join_rows(&plot, &config);

    // Prepend the Y-axis label above the graph body, centered over the
    // full graph width (left_pad + len_max).
    if let Some(ref label) = config.y_axis_label {
        let mut result = format!("{}{}", label, config.line_ending);
        result.push_str(&lines);
        lines = result;
    }

    if config.x_axis_range.is_some() {
        add_x_axis(&mut lines, &config, len_max, left_pad);
    }

    if !config.caption.is_empty() {
        render_caption(&mut lines, &config, len_max, left_pad);
    }

    if !config.series_legends.is_empty() {
        add_legends(&mut lines, &config, len_max, left_pad);
    }

    lines
}

// ---------------------------------------------------------------------------
// Helper: get_char_set
// ---------------------------------------------------------------------------

/// Returns the [`CharSet`] for a given series index.
///
/// Falls back to [`DEFAULT_CHAR_SET`] when the series index is beyond the
/// end of `config.series_chars`.
pub(crate) fn get_char_set(config: &Config, series_index: usize) -> CharSet {
    if series_index < config.series_chars.len() {
        return config.series_chars[series_index].clone();
    }
    DEFAULT_CHAR_SET
}

// ---------------------------------------------------------------------------
// Step 0 — config normalization
// ---------------------------------------------------------------------------

/// Applies default values to any config fields that were not explicitly set.
///
/// Specifically: sets `offset` to `3` if it is `0`, and sets `line_ending`
/// to `"\n"` if it is empty. This runs once at the top of [`plot_many`]
/// before any other processing.
fn normalize_config(mut config: Config) -> Config {
    if config.offset == 0 {
        config.offset = 3;
    }
    if config.line_ending.is_empty() {
        config.line_ending = "\n".to_string();
    }
    config
}

// ---------------------------------------------------------------------------
// Step 1 — data preparation
// ---------------------------------------------------------------------------

/// Deep-copies the input slices into owned `Vec<f64>` values, pads shorter
/// series to the length of the longest with `NaN`, and interpolates every
/// series to `config.width` points when a target width is configured.
///
/// Returns the prepared data and the effective column count (`len_max`).
fn prepare_data(data: &[&[f64]], config: &Config) -> (Vec<Vec<f64>>, usize) {
    let mut data: Vec<Vec<f64>> = data.iter().map(|s| s.to_vec()).collect();
    let mut len_max = data.iter().map(Vec::len).max().unwrap_or(0);

    if config.width > 0 {
        for series in data.iter_mut() {
            series.resize(len_max, f64::NAN);
            *series = interpolate_array(series, config.width as u32);
        }
        len_max = config.width;
    }

    // If a moving average window is configured, compute it from the first
    // series and append it as an additional series. It is computed after
    //  interpolation, so the window applies to the final column count.
    if let Some(window) = config.moving_average_window {
        if !data.is_empty() {
            let ma = utils::moving_average(&data[0], window);
            data.push(ma);
        }
    }

    (data, len_max)
}

// ---------------------------------------------------------------------------
// Step 2 — bounds calculation
// ---------------------------------------------------------------------------

/// All scaled integer bounds derived from the data range.
///
/// These values are computed once in [`calculate_bounds`] and passed by
/// reference to every subsequent rendering function. The scaled integers
/// (`intmin2`, `intmax2`, `min2`) are used to map floating-point data values
/// to discrete grid row indices.
pub(crate) struct Bounds {
    /// The global minimum value across all series (after bound overrides).
    minimum: f64,
    /// The global maximum value across all series (after bound overrides).
    maximum: f64,
    /// The difference between maximum and minimum.
    interval: f64,
    /// Scaling factor mapping data values to grid rows.
    ratio: f64,
    /// Total number of grid rows (= intmax2 - intmin2).
    rows: usize,
    /// Scaled integer representation of the minimum value.
    intmin2: isize,
    /// Scaled integer representation of the maximum value.
    intmax2: isize,
    /// Rounded scaled minimum, used as a baseline for row mapping.
    min2: f64,
}

/// Scans every series to find the global min/max, applies any configured
/// lower/upper bound overrides, then computes the scaled integer bounds used
/// throughout the rendering pipeline.
///
/// The ratio and scaled integers are what allow floating-point data values to
/// be mapped to discrete grid row indices consistently across all rendering
/// functions.
fn calculate_bounds(data: &[Vec<f64>], config: &Config) -> Bounds {
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;

    for series in data.iter() {
        match min_max_float64_slice(series) {
            Some((min_v, max_v)) => {
                if min_v < minimum { minimum = min_v; }
                if max_v > maximum { maximum = max_v; }
            }
            None => eprintln!("warning: series contained no finite values"),
        }
    }

    if let Some(lb) = config.lower_bound {
        if lb < minimum { minimum = lb; }
    }
    if let Some(ub) = config.upper_bound {
        if ub > maximum { maximum = ub; }
    }

    debug_assert!(maximum >= minimum, "maximum must be >= minimum");

    let interval = maximum - minimum;
    let height = if config.height > 0 { config.height } else { calculate_height(interval) };
    let ratio = if interval != 0.0 { height as f64 / interval } else { 1.0 };

    let min2    = utils::round(minimum * ratio);
    let max2    = utils::round(maximum * ratio);
    let intmin2 = min2.round() as isize;
    let intmax2 = max2.round() as isize;
    let rows    = (intmax2 - intmin2).unsigned_abs();

    Bounds { minimum, maximum, interval, ratio, rows, intmin2, intmax2, min2 }
}

// ---------------------------------------------------------------------------
// Step 3 — grid initialization
// ---------------------------------------------------------------------------

/// Allocates a blank `(rows + 1) × width` grid of default [`Cell`] values.
///
/// Every cell starts as a space character with the default (no) color.
/// Rendering functions overwrite specific cells; `join_rows` reads them all.
fn init_grid(rows: usize, width: usize) -> Vec<Vec<Cell>> {
    vec![vec![Cell::default(); width]; rows + 1]
}

// ---------------------------------------------------------------------------
// Step 4 — precision calculation
// ---------------------------------------------------------------------------

/// Computes the number of decimal places to use for Y-axis labels.
///
/// When `config.precision` is set explicitly, that value is used directly.
/// Otherwise, the library applies a heuristic: extra decimal places are added
/// for very small values (to avoid losing meaningful digits), and large values
/// default to zero decimal places (integers are sufficient).
fn calculate_precision(maximum: f64, minimum: f64, config: &Config) -> usize {
    let mut precision = config.precision.unwrap_or(2);
    let mut log_maximum = maximum.abs().max(minimum.abs()).log10();

    if minimum == 0.0 && maximum == 0.0 {
        log_maximum = -1.0;
    }

    if log_maximum < 0.0 {
        if log_maximum.fract() != 0.0 {
            precision += log_maximum.abs() as usize;
        } else {
            precision += (log_maximum.abs() - 1.0) as usize;
        }
    } else if log_maximum > 2.0 && config.precision.is_none() {
        precision = 0;
    }

    precision
}

// ---------------------------------------------------------------------------
// Step 5 — Y-axis magnitudes
// ---------------------------------------------------------------------------

/// Computes the real-valued magnitude for each grid row and determines the
/// maximum Y-axis label width.
///
/// Iterates over every row from `intmin2` to `intmax2`, computing the data
/// value that corresponds to that row. When a custom `y_axis_value_formatter`
/// is set, the rendered label string is measured instead of the raw float, so
/// the Y-axis margin is always wide enough regardless of the formatter output.
///
/// Returns `(magnitudes, max_width)` where `magnitudes[0]` corresponds to the
/// top row of the grid and `max_width` is the column count needed for the
/// widest label.
fn calculate_y_axis_magnitudes(
    bounds: &Bounds,
    precision: usize,
    config: &Config,
) -> (Vec<f64>, usize) {
    let mut max_num_length = format!("{:.prec$}", bounds.maximum, prec = precision)
        .chars()
        .count();
    let min_num_length = format!("{:.prec$}", bounds.minimum, prec = precision)
        .chars()
        .count();

    if config.y_axis_value_formatter.is_some() {
        max_num_length = 0;
    }

    let mut magnitudes = Vec::with_capacity(bounds.rows + 1);

    for y in bounds.intmin2..=bounds.intmax2 {
        let magnitude = if bounds.rows > 0 && bounds.interval > 0.0 {
            bounds.maximum
                - (((y - bounds.intmin2) as f64 * bounds.interval) / bounds.rows as f64)
        } else if bounds.interval == 0.0 {
            bounds.minimum
        } else {
            y as f64
        };

        magnitudes.push(magnitude);

        if let Some(formatter) = &config.y_axis_value_formatter {
            let l = formatter(magnitude).chars().count();
            if l > max_num_length {
                max_num_length = l;
            }
        }
    }

    let max_width = if config.y_axis_value_formatter.is_some() {
        max_num_length
    } else {
        max_num_length.max(min_num_length)
    };

    (magnitudes, max_width)
}

// ---------------------------------------------------------------------------
// Step 6 — render Y-axis into grid
// ---------------------------------------------------------------------------

/// Writes Y-axis labels and tick characters into the left margin of the grid.
///
/// For each row, the corresponding magnitude is formatted as a right-aligned
/// string and written into the label area. The `┤` tick character is written
/// at the column immediately left of the data area. If a custom
/// `y_axis_value_formatter` is configured, it is used instead of the default
/// float formatting.
fn render_y_axis(
    plot: &mut Vec<Vec<Cell>>,
    magnitudes: &[f64],
    max_width: usize,
    precision: usize,
    config: &Config,
) {
    for (w, &magnitude) in magnitudes.iter().enumerate() {
        let label = if let Some(formatter) = &config.y_axis_value_formatter {
            format!("{:>width$}", formatter(magnitude), width = max_width + 1)
        } else {
            format!("{:>width$.prec$}", magnitude, width = max_width + 1, prec = precision)
        };

        let h = ((config.offset as f64) - (label.chars().count() as f64))
            .max(0.0) as usize;

        plot[w][h].text  = label;
        plot[w][h].color = config.label_color;
        plot[w][config.offset - 1].text  = "┤".to_string();
        plot[w][config.offset - 1].color = config.axis_color;
    }
}

// ---------------------------------------------------------------------------
// Step 7 — render series into grid
// ---------------------------------------------------------------------------

/// Draws arc and line characters for every data series into the plot grid.
///
/// For each consecutive pair of data points, the appropriate character is
/// selected and written into the grid:
///
/// - Both points equal → horizontal character
/// - One point higher → corner arcs at the endpoints, vertical fill between
/// - Either point is `NaN` → start cap or end cap at the boundary
/// - Both points `NaN` → nothing drawn
///
/// Series are rendered in order. Later series overwrite earlier ones where
/// they share the same cell. Colors are applied to every row touched by a
/// segment, including the arc endpoints.
fn render_series(
    plot: &mut Vec<Vec<Cell>>,
    data: &[Vec<f64>],
    bounds: &Bounds,
    len_max: usize,
    config: &Config,
) {
    let _ = len_max;

    for (i, series) in data.iter().enumerate() {
        let color = config.series_colors.get(i).copied().unwrap_or(AnsiColor::DEFAULT);
        let char_set = get_char_set(config, i);
        let (mut y0, mut y1): (usize, usize);

        if !series[0].is_nan() {
            y0 = ((series[0] * bounds.ratio).round() - bounds.min2) as usize;
            plot[bounds.rows - y0][config.offset - 1].text  = "┼".to_string();
            plot[bounds.rows - y0][config.offset - 1].color = config.axis_color;
        }

        for (x, window) in series.windows(2).enumerate() {
            let (d0, d1) = (window[0], window[1]);

            if d0.is_nan() && d1.is_nan() { continue; }

            if !d0.is_nan() && d1.is_nan() {
                y0 = ((d0 * bounds.ratio).round() - bounds.intmin2 as f64) as usize;
                plot[bounds.rows - y0][x + config.offset].text  = char_set.end_cap.to_string();
                plot[bounds.rows - y0][x + config.offset].color = color;
                continue;
            }

            if d0.is_nan() && !d1.is_nan() {
                y1 = ((d1 * bounds.ratio).round() - bounds.intmin2 as f64) as usize;
                plot[bounds.rows - y1][x + config.offset].text  = char_set.start_cap.to_string();
                plot[bounds.rows - y1][x + config.offset].color = color;
                continue;
            }

            y0 = ((d0 * bounds.ratio).round() - bounds.intmin2 as f64) as usize;
            y1 = ((d1 * bounds.ratio).round() - bounds.intmin2 as f64) as usize;

            if y0 == y1 {
                plot[bounds.rows - y0][x + config.offset].text = char_set.horizontal.to_string();
            } else {
                if y0 > y1 {
                    plot[bounds.rows - y1][x + config.offset].text = char_set.arc_up_right.to_string();
                    plot[bounds.rows - y0][x + config.offset].text = char_set.arc_down_left.to_string();
                } else {
                    plot[bounds.rows - y1][x + config.offset].text = char_set.arc_down_right.to_string();
                    plot[bounds.rows - y0][x + config.offset].text = char_set.arc_up_left.to_string();
                }

                let lo = y0.min(y1) + 1;
                let hi = y0.max(y1);
                for y in lo..hi {
                    plot[bounds.rows - y][x + config.offset].text = char_set.vertical_line.to_string();
                }
            }

            let lo = y0.min(y1);
            let hi = y0.max(y1);
            for y in lo..=hi {
                plot[bounds.rows - y][x + config.offset].color = color;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Step 8 — join rows into a String
// ---------------------------------------------------------------------------

/// Flattens the 2-D cell grid into a single `String`.
///
/// Rows are joined with `config.line_ending`. Within each row, ANSI escape
/// codes are emitted only when the active color changes — this avoids bloating
/// the output with redundant codes. If the last active color on a row is not
/// the default, a reset code is appended before moving to the next row.
/// Trailing spaces are trimmed from every row.
fn join_rows(plot: &Vec<Vec<Cell>>, config: &Config) -> String {
    let mut lines = String::new();

    for (h, row) in plot.iter().enumerate() {
        if h != 0 {
            lines.push_str(&config.line_ending);
        }

        let mut row_str = String::new();
        let mut current_color = AnsiColor::DEFAULT;

        for cell in row.iter() {
            if cell.color != current_color {
                current_color = cell.color;
                row_str.push_str(&current_color.to_string());
            }
            row_str.push_str(&cell.text);
        }

        if current_color != AnsiColor::DEFAULT {
            row_str.push_str(&AnsiColor::DEFAULT.to_string());
        }

        lines.push_str(row_str.trim_end_matches(' '));
    }

    lines
}

// ---------------------------------------------------------------------------
// Step 9 — caption rendering
// ---------------------------------------------------------------------------

/// Appends the caption string below the plot body.
///
/// The caption is indented by `left_pad` columns and then centered over the
/// data area. If a caption color is configured, ANSI escape codes wrap the
/// caption text.
fn render_caption(lines: &mut String, config: &Config, len_max: usize, left_pad: usize) {
    lines.push_str(&config.line_ending);
    lines.push_str(&" ".repeat(left_pad));

    if config.caption.len() < len_max {
        lines.push_str(&" ".repeat((len_max - config.caption.len()) / 2));
    }

    if config.caption_color != AnsiColor::DEFAULT {
        lines.push_str(&config.caption_color.to_string());
    }

    lines.push_str(&config.caption);

    if config.caption_color != AnsiColor::DEFAULT {
        lines.push_str(&AnsiColor::DEFAULT.to_string());
    }
}
