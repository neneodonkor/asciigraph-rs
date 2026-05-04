// Config — the builder-pattern configuration struct for graph rendering.

use crate::colors::AnsiColor;
use crate::options::charset::{CharSet};
use crate::options::extensions::{ZeroLine, Threshold, StatAnnotations};

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

    /// Number of tick marks on the X-axis. `0` means auto-calculate.
    /// Minimum when set explicitly is `2`.
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
    /// is also configured.
    pub x_axis_label: Option<String>,

    /// Optional statistical annotations rendered as horizontal reference lines
    /// at computed values — minimum, maximum, mean, median, and standard deviation.
    /// Set via [`Config::stat_annotations()`].
    pub stat_annotations: Option<StatAnnotations>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width:                  0,
            height:                 0,
            lower_bound:            None,
            upper_bound:            None,
            offset:                 3,
            caption:                String::new(),
            precision:              None,
            caption_color:          AnsiColor::DEFAULT,
            axis_color:             AnsiColor::DEFAULT,
            label_color:            AnsiColor::DEFAULT,
            series_colors:          Vec::new(),
            series_legends:         Vec::new(),
            line_ending:            String::from("\n"),
            series_chars:           Vec::new(),
            x_axis_tick_count:      0,
            x_axis_range:           None,
            x_axis_value_formatter: None,
            y_axis_value_formatter: None,
            zero_line:              None,
            thresholds:             Vec::new(),
            moving_average_window:  None,
            x_axis_label:           None,
            y_axis_label:           None,
            stat_annotations:       None,
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
    ///
    /// [`create_char_set`]: crate::options::create_char_set
    pub fn series_chars(mut self, cs: &[CharSet]) -> Self {
        self.series_chars = cs.to_vec();
        self
    }

    /// Sets the number of tick marks on the X-axis, overriding the automatic
    /// calculation.
    ///
    /// When this is not called, the library automatically determines a sensible
    /// tick count based on the available graph width and the estimated width of
    /// the tick labels. The minimum accepted value is `2` — values below `2`
    /// are ignored and the automatic calculation is used instead.
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
    /// body. The number of ticks is calculated automatically — call
    /// [`Config::x_axis_tick_count()`] to override.
    ///
    /// # Example
    ///
    /// ```rust
    /// use asciigraph::Config;
    ///
    /// let config = Config::default().x_axis_range(0.0, 100.0);
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
    /// The line is only visible when the data range straddles zero. Series arc
    /// characters always render on top of the zero line.
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
    /// Call this method multiple times to add more than one threshold. Series
    /// arc characters always render on top of threshold lines.
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

    /// Enables a moving average overlay rendered as an additional series.
    ///
    /// The smoothed series is drawn on top of the original data using the next
    /// available series color and character set. A window of `0` or `1` has no
    /// effect.
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

    /// Sets a descriptive label for the X-axis.
    ///
    /// Rendered inline on the same row as the axis line, to the right of the
    /// tick marks. Only visible when [`Config::x_axis_range()`] is also set.
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

    /// Sets a descriptive label for the Y-axis.
    ///
    /// Rendered flush left above the graph body.
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

    /// Enables statistical annotations as horizontal reference lines across
    /// the data area.
    ///
    /// Supports minimum, maximum, mean, median, and standard deviation.
    /// Annotations are rendered before the series so series arc characters
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