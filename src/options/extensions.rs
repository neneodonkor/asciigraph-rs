// Overlay annotation types — ZeroLine, Threshold, and StatAnnotations.
//
// All three structs serve the same conceptual role: opt-in horizontal
// reference lines drawn on top of the graph at computed or user-specified
// Y values. Keeping them together makes it easy to find and reason about
// the annotation surface as a unit.

use crate::colors::AnsiColor;
use crate::options::charset::DEFAULT_CHAR_SET;

// ---------------------------------------------------------------------------
// ZeroLine
// ---------------------------------------------------------------------------

/// A horizontal reference line drawn at Y = 0.0 across the data area.
///
/// The zero line is only rendered when the data range straddles zero — if all
/// values are positive or all negative, this option has no effect. It is
/// rendered before the data series so that series arc characters always appear
/// on top.
///
/// # Example
///
/// ```rust
/// use asciigraph::{plot, Config, ZeroLine, AnsiColor};
///
/// let data = vec![-3.0, -1.0, 0.0, 1.0, 3.0];
/// let graph = plot(&data, Config::default().zero_line(ZeroLine::new()));
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy)]
pub struct ZeroLine {
    /// The ANSI color used to render the zero line.
    /// Defaults to [`AnsiColor::DEFAULT`] (no color).
    pub color: AnsiColor,

    /// The character used to draw the zero line.
    /// Defaults to `─` ([`DEFAULT_CHAR_SET`]`.horizontal`).
    pub character: char,
}

impl ZeroLine {
    /// Creates a zero line using the default horizontal character and no color.
    pub fn new() -> Self {
        ZeroLine {
            color: AnsiColor::DEFAULT,
            character: DEFAULT_CHAR_SET.horizontal,
        }
    }

    /// Creates a zero line rendered in a specific ANSI color.
    /// Uses the default horizontal character.
    pub fn with_color(color: AnsiColor) -> Self {
        ZeroLine {
            color,
            character: DEFAULT_CHAR_SET.horizontal,
        }
    }

    /// Creates a zero line with both a custom character and a custom ANSI color.
    pub fn with_char_and_color(character: char, color: AnsiColor) -> Self {
        ZeroLine { color, character }
    }
}

impl Default for ZeroLine {
    fn default() -> Self {
        ZeroLine::new()
    }
}

// ---------------------------------------------------------------------------
// Threshold
// ---------------------------------------------------------------------------

/// A horizontal reference line drawn at a user-specified Y value,
/// associated with a specific data series.
///
/// Threshold lines are rendered as dashed lines (`╌`) across the data area
/// at the given value, making limits, targets, or alert boundaries immediately
/// visible on the graph. Multiple thresholds can be added to a single graph
/// by calling [`Config::threshold()`] repeatedly.
///
/// Each threshold is associated with a series via `series_index`, which
/// defaults to `0` (the first series). Two rules are applied before a
/// threshold is drawn:
///
/// **Visibility rule** — the threshold value must fall within the min/max
/// range of its associated series specifically, not just the global graph
/// range. This means a threshold at Y = 80.0 associated with a series whose
/// values only reach 40.0 will be silently skipped, even if another series
/// on the same graph reaches 90.0.
///
/// **Color inheritance rule** — when no explicit color is set on the
/// threshold (i.e. `color` is [`AnsiColor::DEFAULT`]), the threshold
/// automatically inherits the color of its associated series from
/// `Config::series_colors`. An explicitly set color always takes priority.
///
/// Series arc characters always render on top of threshold lines.
///
/// # Example
///
/// ```rust
/// use asciigraph::{plot_many, Config, Threshold, AnsiColor};
///
/// let s1 = vec![60.0, 75.0, 85.0, 92.0, 78.0, 65.0];
/// let s2 = vec![10.0, 18.0, 25.0, 35.0, 28.0, 15.0];
///
/// let graph = plot_many(
///     &[&s1, &s2],
///     Config::default()
///         .series_colors(&[AnsiColor::BLUE, AnsiColor::GREEN])
///         // Targets series 0 — inherits BLUE from series_colors.
///         .threshold(Threshold::new(80.0))
///         // Targets series 1 — overrides the inherited color.
///         .threshold(Threshold {
///             series_index: 1,
///             ..Threshold::with_color(30.0, AnsiColor::RED)
///         }),
/// );
/// println!("{}", graph);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy)]
pub struct Threshold {
    /// The Y value at which the threshold line is drawn.
    pub value: f64,

    /// The ANSI color used to render the threshold line.
    /// Defaults to [`AnsiColor::DEFAULT`] (no color).
    pub color: AnsiColor,

    /// The character used to draw the threshold line.
    /// Defaults to `╌` ([`DEFAULT_CHAR_SET`]`.dash_horizontal`).
    pub character: char,

    /// The index of the series this threshold is associated with.
    ///
    /// The threshold is only rendered if its value falls within the min/max
    /// range of the series at this index. If the index is out of range or
    /// the threshold value falls outside the series range, the threshold is
    /// silently skipped. When no explicit color is set, the color of the
    /// associated series is inherited automatically.
    ///
    /// Defaults to `0`, which associates the threshold with the first series.
    pub series_index: usize,
}

impl Threshold {
    /// Creates a threshold line at the given Y value using the default dashed
    /// character and no color.
    pub fn new(value: f64) -> Self {
        Threshold {
            value,
            color: AnsiColor::DEFAULT,
            character: DEFAULT_CHAR_SET.dash_horizontal,
            series_index: 0,
        }
    }

    /// Creates a threshold line at the given Y value rendered in a specific
    /// ANSI color. Uses the default dashed character.
    pub fn with_color(value: f64, color: AnsiColor) -> Self {
        Threshold {
            value,
            color,
            character: DEFAULT_CHAR_SET.dash_horizontal,
            series_index: 0,
        }
    }

    /// Creates a threshold line at the given Y value with both a custom
    /// character and a custom ANSI color.
    pub fn with_char_and_color(value: f64, character: char, color: AnsiColor) -> Self {
        Threshold { value, color, character, series_index: 0 }
    }
}

// ---------------------------------------------------------------------------
// StatAnnotations
// ---------------------------------------------------------------------------

/// Opt-in statistical annotations rendered as horizontal reference lines
/// at computed values across the data area.
///
/// The library computes each statistic from the data automatically — no
/// manual calculation is required. Each annotation is individually
/// controlled by a boolean flag, so you can display any combination of
/// minimum, maximum, mean, median, and standard deviation.
///
/// By default, statistics are computed from the first series (`series_index
/// = 0`). In a multi-series graph, set `series_index` to the index of the
/// series you want to annotate. If the index is out of range, the function
/// falls back to the first series silently.
///
/// Use [`StatAnnotations::new()`] to enable all five annotations at once,
/// or set individual flags to `false` to disable specific ones. All
/// annotations share a single color configured on the struct.
///
/// Annotations are rendered before the series, so series arc characters
/// always appear on top where they overlap.
///
/// # Example
///
/// ```rust
/// use asciigraph::{plot, Config, StatAnnotations, AnsiColor};
///
/// let data = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
///
/// // Enable all annotations with no color.
/// let graph = plot(&data, Config::default().stat_annotations(StatAnnotations::new()));
///
/// // Enable only min and max in red.
/// let graph = plot(
///     &data,
///     Config::default().stat_annotations(StatAnnotations {
///         show_min:     true,
///         show_max:     true,
///         show_mean:    false,
///         show_median:  false,
///         show_std_dev: false,
///         series_index: 0,
///         color:        AnsiColor::RED,
///     }),
/// );
///
/// // Annotate the second series in a multi-series graph.
/// let graph = plot(
///     &data,
///     Config::default().stat_annotations(StatAnnotations {
///         series_index: 1,
///         ..StatAnnotations::new()
///     }),
/// );
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy)]
pub struct StatAnnotations {
    /// Draws a reference line at the minimum value of the dataset.
    pub show_min: bool,

    /// Draws a reference line at the maximum value of the dataset.
    pub show_max: bool,

    /// Draws a reference line at the mean (average) value of the dataset.
    pub show_mean: bool,

    /// Draws a reference line at the median value of the dataset.
    pub show_median: bool,

    /// Draws a reference line at one standard deviation above and below
    /// the mean, giving a visual indication of the data's spread.
    pub show_std_dev: bool,

    /// The ANSI color used to render all annotation lines.
    /// Defaults to [`AnsiColor::DEFAULT`] (no color).
    pub color: AnsiColor,

    /// The index of the series to compute statistics from.
    ///
    /// In a single-series graph this is always `0`. In a multi-series graph,
    /// set this to the index of the series you want to annotate. If the index
    /// is out of range, the function falls back to the first series silently.
    ///
    /// Use struct update syntax to set this field without changing anything else:
    ///
    /// ```rust
    /// use asciigraph::StatAnnotations;
    ///
    /// let annotations = StatAnnotations {
    ///     series_index: 1,
    ///     ..StatAnnotations::new()
    /// };
    /// ```
    pub series_index: usize,
}

impl StatAnnotations {
    /// Creates a `StatAnnotations` value with all five annotations enabled,
    /// no color, and targeting the first series (`series_index = 0`).
    pub fn new() -> Self {
        StatAnnotations {
            show_min:     true,
            show_max:     true,
            show_mean:    true,
            show_median:  true,
            show_std_dev: true,
            color:        AnsiColor::DEFAULT,
            series_index: 0,
        }
    }

    /// Creates a `StatAnnotations` value with all five annotations enabled,
    /// rendered in a specific ANSI color, and targeting the first series.
    ///
    /// For multi-series graphs, override `series_index` with struct update syntax:
    ///
    /// ```rust
    /// use asciigraph::{StatAnnotations, AnsiColor};
    ///
    /// let annotations = StatAnnotations {
    ///     series_index: 1,
    ///     ..StatAnnotations::with_color(AnsiColor::YELLOW)
    /// };
    /// ```
    pub fn with_color(color: AnsiColor) -> Self {
        StatAnnotations {
            show_min:     true,
            show_max:     true,
            show_mean:    true,
            show_median:  true,
            show_std_dev: true,
            series_index: 0,
            color,
        }
    }
}

impl Default for StatAnnotations {
    fn default() -> Self {
        StatAnnotations::new()
    }
}