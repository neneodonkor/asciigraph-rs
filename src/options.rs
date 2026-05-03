// Options

use std::string::ToString;
use crate::color::AnsiColor;

// ---------------------------------------------------------------------------
// CharSet
// ---------------------------------------------------------------------------

/// Defines the set of characters used to draw a data series on the graph.
///
/// Each field controls a specific part of the line rendering — horizontal
/// runs, vertical segments, corner arcs, NaN gap caps, axis corners, and
/// tick marks. Swap out individual characters to change the visual style of
/// a series without affecting the rendering logic.
///
/// Use [`create_char_set`] to create a uniform set where every character is
/// the same (e.g. `*` or `•`). Use struct update syntax (`..Default::default()`)
/// to override only the fields you care about while keeping the rest as the
/// defaults from [`DEFAULT_CHAR_SET`].
///
/// # Example
///
/// ```rust
/// use asciigraph::options::CharSet;
///
/// // Override only the horizontal and vertical characters.
/// let partial = CharSet {
///     horizontal: '=',
///     vertical_line: '|',
///     ..Default::default()
/// };
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct CharSet {
    /// Horizontal line character used for flat segments. Default: `─`
    pub horizontal: char,

    /// Vertical line character used for steep segments. Default: `│`
    pub vertical_line: char,

    /// Corner arc going down and to the right (rising series). Default: `╭`
    pub arc_down_right: char,

    /// Corner arc going down and to the left (falling series). Default: `╮`
    pub arc_down_left: char,

    /// Corner arc going up and to the right (falling series). Default: `╰`
    pub arc_up_right: char,

    /// Corner arc going up and to the left (rising series). Default: `╯`
    pub arc_up_left: char,

    /// End cap drawn at the last finite point before a NaN gap. Default: `╴`
    pub end_cap: char,

    /// Start cap drawn at the first finite point after a NaN gap. Default: `╶`
    pub start_cap: char,

    /// Bottom-left corner character for the X-axis. Default: `└`
    pub up_right: char,

    /// Tick mark character used on the X-axis. Default: `┬`
    pub down_horizontal: char,

    /// Dashed horizontal character used for threshold lines. Default: `╌`
    pub dash_horizontal: char,

    /// Dashed horizontal character used for mean annotation. Default: ┄
    pub double_dash_horizontal: char,

    /// Heavy dashed horizontal character used for median annotation. Default: ╍
    pub heavy_dash_horizontal: char,

    /// Dotted horizontal character used for standard deviation annotation. Default: ·
    pub dot_horizontal: char,
}

impl Default for CharSet {
    fn default() -> Self {
        CharSet {
            horizontal:             DEFAULT_CHAR_SET.horizontal,
            vertical_line:          DEFAULT_CHAR_SET.vertical_line,
            arc_down_right:         DEFAULT_CHAR_SET.arc_down_right,
            arc_down_left:          DEFAULT_CHAR_SET.arc_down_left,
            arc_up_right:           DEFAULT_CHAR_SET.arc_up_right,
            arc_up_left:            DEFAULT_CHAR_SET.arc_up_left,
            end_cap:                DEFAULT_CHAR_SET.end_cap,
            start_cap:              DEFAULT_CHAR_SET.start_cap,
            up_right:               DEFAULT_CHAR_SET.up_right,
            down_horizontal:        DEFAULT_CHAR_SET.down_horizontal,
            dash_horizontal:        DEFAULT_CHAR_SET.dash_horizontal,
            double_dash_horizontal: DEFAULT_CHAR_SET.double_dash_horizontal,
            heavy_dash_horizontal:  DEFAULT_CHAR_SET.heavy_dash_horizontal,
            dot_horizontal:         DEFAULT_CHAR_SET.dot_horizontal,
        }
    }
}

/// The default box-drawing character set used when no custom [`CharSet`] is provided.
pub const DEFAULT_CHAR_SET: CharSet = CharSet {
    horizontal:             '─',
    vertical_line:          '│',
    arc_down_right:         '╭',
    arc_down_left:          '╮',
    arc_up_right:           '╰',
    arc_up_left:            '╯',
    end_cap:                '╴',
    start_cap:              '╶',
    up_right:               '└',
    down_horizontal:        '┬',
    dash_horizontal:        '╌',
    double_dash_horizontal: '┄',
    heavy_dash_horizontal:  '╍',
    dot_horizontal:         '·',
};

/// Creates a [`CharSet`] where every character is set to the same value.
///
/// Useful for simple, uniform plot styles such as `*`, `•`, or `#`, where
/// the distinction between horizontal runs, vertical segments, and arcs is
/// not important — every position in the series uses the same character.
///
/// # Example
///
/// ```rust
/// use asciigraph::options::create_char_set;
///
/// let asterisk = create_char_set('*');
/// let dot = create_char_set('•');
/// ```
pub fn create_char_set(character: char) -> CharSet {
    CharSet {
        horizontal:             character,
        vertical_line:          character,
        arc_down_right:         character,
        arc_down_left:          character,
        arc_up_right:           character,
        arc_up_left:            character,
        end_cap:                character,
        start_cap:              character,
        up_right:               character,
        down_horizontal:        character,
        dash_horizontal:        character,
        double_dash_horizontal: character,
        heavy_dash_horizontal:  character,
        dot_horizontal:         character,
    }
}

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

/// Configuration for controlling the appearance and behavior of a graph.
///
/// `Config` uses a builder pattern — start with [`Config::default()`] and
/// chain the methods for the options you want to set. Every method consumes
/// and returns `Self`, so calls can be chained fluently.
///
/// # Example
///
/// ```rust
/// use asciigraph::{plot, Config, AnsiColor};
///
/// let data = vec![1.0, 2.0, 3.0, 2.0, 1.0];
/// let graph = plot(
///     &data,
///     Config::default()
///         .height(5)
///         .caption("My Graph")
///         .axis_color(AnsiColor::GREEN),
/// );
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Config {
    /// Target width of the data area in columns. `0` means auto-size to the
    /// number of data points.
    pub width: usize,

    /// Target height of the graph in rows. `0` means auto-size based on the
    /// data range.
    pub height: usize,

    /// Optional lower bound for the Y-axis. Ignored if the data minimum is
    /// already below this value.
    pub lower_bound: Option<f64>,

    /// Optional upper bound for the Y-axis. Ignored if the data maximum is
    /// already above this value.
    pub upper_bound: Option<f64>,

    /// Number of columns reserved for the Y-axis label area. Defaults to `3`.
    pub offset: usize,

    /// Caption string rendered below the graph body.
    pub caption: String,

    /// Number of decimal places for Y-axis labels. `None` means auto-detect.
    pub precision: Option<usize>,

    /// ANSI color for the caption text.
    pub caption_color: AnsiColor,

    /// ANSI color for axis lines and tick characters.
    pub axis_color: AnsiColor,

    /// ANSI color for Y-axis labels.
    pub label_color: AnsiColor,

    /// Per-series ANSI colors. The first color applies to the first series,
    /// the second to the second, and so on.
    pub series_colors: Vec<AnsiColor>,

    /// Per-series legend labels rendered below the graph.
    pub series_legends: Vec<String>,

    /// Line ending sequence. Defaults to `"\n"`. Use `"\r\n"` for Windows
    /// raw terminals.
    pub line_ending: String,

    /// Per-series character sets. Falls back to [`DEFAULT_CHAR_SET`] for any
    /// series that does not have an explicit entry.
    pub series_chars: Vec<CharSet>,

    /// Number of tick marks on the X-axis. Defaults to `5` when an X-axis
    /// range is set. Minimum is `2`.
    pub x_axis_tick_count: usize,

    /// The `[min, max]` domain mapped onto the X-axis. Setting this enables
    /// the X-axis.
    pub x_axis_range: Option<[f64; 2]>,

    /// Custom formatter for X-axis tick labels. Accepts any closure of the
    /// form `Fn(f64) -> String`.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub x_axis_value_formatter: Option<Box<dyn Fn(f64) -> String>>,

    /// Custom formatter for Y-axis labels. Accepts any closure of the form
    /// `Fn(f64) -> String`.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub y_axis_value_formatter: Option<Box<dyn Fn(f64) -> String>>,

    /// Optional zero line drawn at Y = 0.0 across the data area.
    pub zero_line: Option<ZeroLine>,

    /// Horizontal reference lines drawn at user-specified Y values.
    pub thresholds: Vec<Threshold>,

    /// Window size for the moving average overlay. `None` means disabled.
    pub moving_average_window: Option<usize>,

    /// Descriptive label rendered flush left above the graph body.
    /// Set via [`Config::y_axis_label()`].
    pub y_axis_label: Option<String>,

    /// Descriptive label rendered inline on the same row as the X-axis line,
    /// to the right of the tick marks. Only visible when [`Config::x_axis_range()`]
    /// is also configured. Set via [`Config::x_axis_label()`].
    pub x_axis_label: Option<String>,

    /// Optional statistical annotations rendered as horizontal reference lines
    /// at computed values — minimum, maximum, mean, median, and standard deviation.
    /// Set via [`Config::stat_annotations()`].
    pub stat_annotations: Option<StatAnnotations>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 0,
            height: 0,
            lower_bound: None,
            upper_bound: None,
            offset: 3,
            caption: String::new(),
            precision: None,
            caption_color: AnsiColor::DEFAULT,
            axis_color: AnsiColor::DEFAULT,
            label_color: AnsiColor::DEFAULT,
            series_colors: Vec::new(),
            series_legends: Vec::new(),
            line_ending: String::from("\n"),
            series_chars: Vec::new(),
            x_axis_tick_count: 0,
            x_axis_range: None,
            x_axis_value_formatter: None,
            y_axis_value_formatter: None,
            zero_line: None,
            thresholds: Vec::new(),
            moving_average_window: None,
            x_axis_label: None,
            y_axis_label: None,
            stat_annotations: None,
        }
    }
}

impl Config {
    /// Sets the graph width in columns.
    ///
    /// When set to a positive value, the input data is interpolated to produce
    /// exactly this many data columns regardless of how many points were
    /// provided. Pass `0` to auto-size the width to the number of data points.
    pub fn width(mut self, w: usize) -> Self {
        self.width = w;
        self
    }

    /// Sets the graph height in rows.
    ///
    /// Pass `0` to auto-size the height based on the data range, which is the
    /// default behavior.
    pub fn height(mut self, h: usize) -> Self {
        self.height = h;
        self
    }

    /// Sets an optional lower bound for the Y-axis.
    ///
    /// This value is only applied if it is lower than the actual data minimum.
    /// It will not compress the visible range — it can only expand it downward.
    pub fn lower_bound(mut self, min: f64) -> Self {
        self.lower_bound = Some(min);
        self
    }

    /// Sets an optional upper bound for the Y-axis.
    ///
    /// This value is only applied if it is higher than the actual data maximum.
    /// It will not compress the visible range — it can only expand it upward.
    pub fn upper_bound(mut self, max: f64) -> Self {
        self.upper_bound = Some(max);
        self
    }

    /// Sets the number of columns reserved for the Y-axis label area.
    ///
    /// Increase this value if your Y-axis labels are wider than the default
    /// allows. Defaults to `3` when not set.
    pub fn offset(mut self, off: usize) -> Self {
        self.offset = off;
        self
    }

    /// Sets the number of decimal places used in Y-axis labels.
    ///
    /// When not set, the library auto-detects appropriate precision based
    /// on the data range — more decimal places for small values, fewer for
    /// large ones.
    pub fn precision(mut self, p: usize) -> Self {
        self.precision = Some(p);
        self
    }

    /// Sets the caption rendered below the graph body.
    ///
    /// The caption is centered over the data area. Leading and trailing
    /// whitespace is trimmed before rendering.
    pub fn caption(mut self, c: &str) -> Self {
        self.caption = c.trim().to_string();
        self
    }

    /// Sets the ANSI color for the caption text.
    pub fn caption_color(mut self, color: AnsiColor) -> Self {
        self.caption_color = color;
        self
    }

    /// Sets the ANSI color for axis lines and tick characters.
    pub fn axis_color(mut self, color: AnsiColor) -> Self {
        self.axis_color = color;
        self
    }

    /// Sets the ANSI color for Y-axis labels.
    pub fn label_color(mut self, color: AnsiColor) -> Self {
        self.label_color = color;
        self
    }

    /// Sets per-series ANSI colors.
    ///
    /// The first color applies to the first series, the second to the second,
    /// and so on. Series without a corresponding color entry are rendered in
    /// the terminal default color.
    pub fn series_colors(mut self, colors: &[AnsiColor]) -> Self {
        self.series_colors = colors.to_vec();
        self
    }

    /// Sets per-series legend labels rendered below the graph.
    ///
    /// The first label corresponds to the first series, the second to the
    /// second, and so on. Labels are rendered alongside a colored square
    /// marker matching the series color.
    pub fn series_legends(mut self, text: &[&str]) -> Self {
        self.series_legends = text.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Sets the line-ending sequence used between rows.
    ///
    /// Defaults to `"\n"`. Use `"\r\n"` for Windows raw terminals or any
    /// environment that requires CRLF line endings.
    pub fn line_ending(mut self, ending: &str) -> Self {
        self.line_ending = ending.to_string();
        self
    }

    /// Sets per-series character sets.
    ///
    /// The first [`CharSet`] applies to the first series, the second to the
    /// second, and so on. Series without a corresponding entry fall back to
    /// [`DEFAULT_CHAR_SET`]. Use [`create_char_set`] to create a uniform set,
    /// or struct update syntax to override individual characters.
    pub fn series_chars(mut self, cs: &[CharSet]) -> Self {
        self.series_chars = cs.to_vec();
        self
    }

    /// Sets the number of tick marks on the X-axis, overriding the automatic
    /// calculation.
    ///
    /// When this is not called, the library automatically determines a sensible
    /// tick count based on the available graph width and the estimated width of
    /// the tick labels. Call this method only when you need precise control over
    /// the number of ticks — for example, to match a specific grid or to reduce
    /// clutter on a narrow graph.
    ///
    /// The minimum accepted value is `2`. Values below `2` are ignored, and the
    /// automatic calculation is used instead.
    ///
    /// Only takes effect when an X-axis range has been set via
    /// [`Config::x_axis_range()`].
    pub fn x_axis_tick_count(mut self, count: usize) -> Self {
        if count >= 2 {
            self.x_axis_tick_count = count;
        }
        self
    }

    /// Enables the X-axis and maps the domain `[min, max]` onto the plot width.
    ///
    /// Once set, an X-axis line and tick labels are rendered below the graph
    /// body. The number of ticks is calculated automatically based on the
    /// available graph width and the estimated label width — no additional
    /// configuration is required. To override the automatic calculation, call
    /// [`Config::x_axis_tick_count()`] with the desired number of ticks.
    ///
    /// # Example
    ///
    /// ```rust
    /// use asciigraph::Config;
    ///
    /// // Automatic tick count — no x_axis_tick_count call needed.
    /// let config = Config::default().x_axis_range(0.0, 100.0);
    ///
    /// // Explicit tick count — overrides the automatic calculation.
    /// let config = Config::default()
    ///     .x_axis_range(0.0, 100.0)
    ///     .x_axis_tick_count(3);
    /// ```
    pub fn x_axis_range(mut self, min: f64, max: f64) -> Self {
        self.x_axis_range = Some([min, max]);
        self
    }

    /// Sets a custom formatter for X-axis tick labels.
    ///
    /// Accepts any closure of the form `Fn(f64) -> String`. Use this to add
    /// units, control decimal places, or apply any other formatting to the
    /// values printed below the X-axis ticks.
    ///
    /// # Example
    ///
    /// ```rust
    /// use asciigraph::Config;
    ///
    /// let config = Config::default()
    ///     .x_axis_range(0.0, 1000.0)
    ///     .x_axis_value_formatter(Box::new(|v| format!("{:.0}ms", v)));
    /// ```
    pub fn x_axis_value_formatter(mut self, formatter: Box<dyn Fn(f64) -> String>) -> Self {
        self.x_axis_value_formatter = Some(formatter);
        self
    }

    /// Sets a custom formatter for Y-axis labels.
    ///
    /// Accepts any closure of the form `Fn(f64) -> String`. Use this to add
    /// units, convert between scales, or apply any other formatting to the
    /// values printed on the Y-axis.
    ///
    /// # Example
    ///
    /// ```rust
    /// use asciigraph::Config;
    ///
    /// let config = Config::default()
    ///     .y_axis_value_formatter(Box::new(|v| format!("{:.1} GiB", v / 1024.0)));
    /// ```
    pub fn y_axis_value_formatter(mut self, formatter: Box<dyn Fn(f64) -> String>) -> Self {
        self.y_axis_value_formatter = Some(formatter);
        self
    }

    /// Enables a horizontal reference line at Y = 0.0 across the data area.
    ///
    /// The line is drawn using the character and color specified by the given
    /// [`ZeroLine`] value. Use [`ZeroLine::new()`] for an uncolored line, or
    /// [`ZeroLine::with_color()`] to render it in a specific ANSI color.
    ///
    /// The zero line is only visible when the data range straddles zero — that
    /// is, when the minimum is negative and the maximum is positive. If zero
    /// falls outside the plotted range, this option has no effect.
    ///
    /// Series arc characters always render on top of the zero line.
    ///
    /// # Example
    ///
    /// ```rust
    /// use asciigraph::{plot, Config, ZeroLine, AnsiColor};
    ///
    /// let data = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
    /// let graph = plot(
    ///     &data,
    ///     Config::default().zero_line(ZeroLine::with_color(AnsiColor::RED)),
    /// );
    /// ```
    pub fn zero_line(mut self, zero_line: ZeroLine) -> Self {
        self.zero_line = Some(zero_line);
        self
    }

    /// Adds a horizontal reference line at a user-specified Y value.
    ///
    /// Call this method multiple times to add more than one threshold line.
    /// Each threshold is rendered independently with its own value, color,
    /// and character. Thresholds whose value falls outside the visible Y range
    /// are silently ignored.
    ///
    /// Series arc characters always render on top of threshold lines.
    ///
    /// # Example
    ///
    /// ```rust
    /// use asciigraph::{plot, Config, Threshold, AnsiColor};
    ///
    /// let data = vec![60.0, 70.0, 85.0, 92.0, 78.0, 65.0];
    /// let graph = plot(
    ///     &data,
    ///     Config::default()
    ///         .threshold(Threshold::with_color(80.0, AnsiColor::YELLOW))
    ///         .threshold(Threshold::with_color(90.0, AnsiColor::RED)),
    /// );
    /// ```
    pub fn threshold(mut self, t: Threshold) -> Self {
        self.thresholds.push(t);
        self
    }

    /// Enables a moving average overlay rendered as an additional series on
    /// the graph.
    ///
    /// The moving average is computed over a sliding window of `window` points
    /// centered on each data point. The smoothed series is drawn on top of the
    /// original data using the next available series color and character set.
    ///
    /// A window of 0 or 1 has no effect. If the window is larger than the
    /// data length, it is clamped automatically.
    ///
    /// # Example
    ///
    /// ```rust
    /// use asciigraph::{plot, Config};
    ///
    /// let data = vec![3.0, 1.0, 5.0, 2.0, 4.0, 1.0, 6.0, 2.0, 5.0, 1.0];
    /// let graph = plot(&data, Config::default().moving_average(3));
    /// ```
    pub fn moving_average(mut self, window: usize) -> Self {
        self.moving_average_window = Some(window);
        self
    }

    /// Sets a descriptive label for the X-axis, rendered below the tick labels
    /// and centered over the data area.
    ///
    /// The label is only visible when an X-axis range has been set via
    /// [`Config::x_axis_range()`]. If no X-axis range is configured, this
    /// option has no effect.
    ///
    /// # Example
    ///
    /// ```rust
    /// use asciigraph::Config;
    ///
    /// let config = Config::default()
    ///     .x_axis_range(0.0, 100.0)
    ///     .x_axis_label("Time (seconds)");
    /// ```
    pub fn x_axis_label(mut self, label: &str) -> Self {
        self.x_axis_label = Some(label.to_string());
        self
    }

    /// Sets a descriptive label for the Y-axis, rendered horizontally above
    /// the graph body and centered over the full graph width.
    ///
    /// # Example
    ///
    /// ```rust
    /// use asciigraph::Config;
    ///
    /// let config = Config::default().y_axis_label("Memory (MB)");
    /// ```
    pub fn y_axis_label(mut self, label: &str) -> Self {
        self.y_axis_label = Some(label.to_string());
        self
    }

    /// Enables statistical annotations rendered as horizontal reference lines
    /// at computed values across the data area.
    ///
    /// The annotations are derived automatically from the data — no manual
    /// value calculation is required. Each annotation is individually opt-in
    /// through the boolean flags on [`StatAnnotations`]. Use
    /// [`StatAnnotations::new()`] to enable all five at once, or construct
    /// a custom [`StatAnnotations`] to enable only specific ones.
    ///
    /// Annotations are rendered before the series, so series arc characters
    /// always appear on top.
    ///
    /// # Example
    ///
    /// ```rust
    /// use asciigraph::{plot, Config, StatAnnotations, AnsiColor};
    ///
    /// let data = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
    /// let graph = plot(
    ///     &data,
    ///     Config::default()
    ///         .stat_annotations(StatAnnotations::with_color(AnsiColor::YELLOW)),
    /// );
    /// ```
    pub fn stat_annotations(mut self, sa: StatAnnotations) -> Self {
        self.stat_annotations = Some(sa);
        self
    }
}

// END OF CONFIG -----------------------------------------------------------------------------------

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
    /// Defaults to `─` ([`DEFAULT_CHAR_SET::horizontal`]).
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
/// on the same graph reaches 90.0. This prevents thresholds from cluttering
/// a graph when they are not meaningful for their associated series.
///
/// **Color inheritance rule** — when no explicit color is set on the
/// threshold (i.e. `color` is [`AnsiColor::DEFAULT`]), the threshold
/// automatically inherits the color of its associated series from
/// `Config::series_colors`. This creates a natural visual association
/// between a threshold line and the series it belongs to. An explicitly
/// set color always takes priority over the inherited series color.
///
/// Series arc characters always render on top of threshold lines where
/// they overlap.
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
///         // Targets series 1 explicitly — overrides the inherited color.
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
    /// Defaults to `╌` ([`DEFAULT_CHAR_SET::dash_horizontal`]).
    pub character: char,

    /// The index of the series this threshold is associated with.
    ///
    /// The threshold is only rendered if its value falls within the min/max
    /// range of the series at this index. If the index is out of range or
    /// the threshold value falls outside the series range, the threshold is
    /// silently skipped. When no explicit color is set on the threshold, the
    /// color of the associated series is inherited automatically.
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
            series_index: 0
        }
    }

    /// Creates a threshold line at the given Y value rendered in a specific
    /// ANSI color. Uses the default dashed character.
    pub fn with_color(value: f64, color: AnsiColor) -> Self {
        Threshold {
            value,
            color,
            character: DEFAULT_CHAR_SET.dash_horizontal,
            series_index: 0
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
///         show_min: true,
///         show_max: true,
///         show_mean: false,
///         show_median: false,
///         show_std_dev: false,
///         series_index: 0,
///         color: AnsiColor::RED,
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
    /// In a single-series graph this is always `0` and never needs to be
    /// changed. In a multi-series graph, set this to the index of the series
    /// you want to annotate — `0` for the first series, `1` for the second,
    /// and so on.
    ///
    /// If the index is out of range (i.e. greater than the number of series),
    /// the function falls back to the first series silently.
    ///
    /// To set this field without changing anything else, use struct update
    /// syntax rather than a dedicated constructor:
    ///
    /// ```rust
    /// use asciigraph::StatAnnotations;
    ///
    /// let annotations = StatAnnotations {
    ///     series_index: 1, // annotate the second series
    ///     ..StatAnnotations::new()
    /// };
    /// ```
    pub series_index: usize,
}

impl StatAnnotations {
/// Creates a `StatAnnotations` value with all five annotations enabled,
/// no color, and targeting the first series (`series_index = 0`).
///
/// This is the simplest way to add statistical context to a graph —
/// pass `StatAnnotations::new()` to [`Config::stat_annotations()`] and
/// all five reference lines are drawn automatically from the first series.
///
/// For multi-series graphs where you want to annotate a series other
/// than the first, use struct update syntax to override `series_index`:
///
/// ```rust
/// use asciigraph::StatAnnotations;
///
/// let annotations = StatAnnotations {
///     series_index: 2, // annotate the third series
///     ..StatAnnotations::new()
/// };
/// ```
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
/// rendered in a specific ANSI color, and targeting the first series
/// (`series_index = 0`).
///
/// Use this when you want the annotation lines to stand out visually
/// from the series line — for example, rendering the annotations in
/// yellow against a default-colored series makes each reference line
/// immediately obvious.
///
/// For multi-series graphs, override `series_index` using struct update
/// syntax after calling this constructor:
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