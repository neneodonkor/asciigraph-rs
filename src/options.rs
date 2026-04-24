// Options

use std::string::ToString;
use crate::color::AnsiColor;

// CharSet defines the characters used for plotting a series.
#[derive(Debug, Clone)]
pub struct CharSet {
    /// Horizontal line character (default: ─)
    pub horizontal: char,

    /// Vertical line character (default: │)
    pub vertical_line: char,

    /// Arc character going down and right (default: ╭)
    pub arc_down_right: char,

    /// Arc character going down and left (default: ╮)
    pub arc_down_left: char,

    /// Arc character going up and right (default: ╰)
    pub arc_up_right: char,

    /// Arc character going up and left (default: ╯)
    pub arc_up_left: char,

    /// End cap character (default: ╴)
    pub end_cap: char,

    /// Start cap character (default: ╶)
    pub start_cap: char,

    /// Axis corner character (default: └)
    pub up_right: char,

    /// X-axis tick mark character (default: ┬)
    pub down_horizontal: char,
}

impl Default for CharSet {
    fn default() -> Self {
        CharSet {
            horizontal:      DEFAULT_CHAR_SET.horizontal,
            vertical_line:   DEFAULT_CHAR_SET.vertical_line,
            arc_down_right:  DEFAULT_CHAR_SET.arc_down_right,
            arc_down_left:   DEFAULT_CHAR_SET.arc_down_left,
            arc_up_right:    DEFAULT_CHAR_SET.arc_up_right,
            arc_up_left:     DEFAULT_CHAR_SET.arc_up_left,
            end_cap:         DEFAULT_CHAR_SET.end_cap,
            start_cap:       DEFAULT_CHAR_SET.start_cap,
            up_right:        DEFAULT_CHAR_SET.up_right,
            down_horizontal: DEFAULT_CHAR_SET.down_horizontal,
        }
    }
}

// DEFAULT_CHAR_SET provides the default box-drawing characters.
pub const DEFAULT_CHAR_SET: CharSet = CharSet {
    horizontal:      '─',
    vertical_line:   '│',
    arc_down_right:  '╭',
    arc_down_left:   '╮',
    arc_up_right:    '╰',
    arc_up_left:     '╯',
    end_cap:         '╴',
    start_cap:       '╶',
    up_right:        '└',
    down_horizontal: '┬',
};

// create_char_set is a helper function that creates a CharSet with all fields set to the same character.
// This is useful for simple uniform character sets like "*", "•", "#", etc.
pub fn create_char_set(character: char) -> CharSet {
    CharSet {
        horizontal: character,
        vertical_line: character,
        arc_down_right: character,
        arc_down_left: character,
        arc_up_right: character,
        arc_up_left: character,
        end_cap: character,
        start_cap: character,
        up_right: character,
        down_horizontal: character,
    }
}

// Config holds various graph options
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub lower_bound: Option<f64>,
    pub upper_bound: Option<f64>,
    pub offset: usize,
    pub caption: String,
    pub precision: Option<usize>,
    pub caption_color: AnsiColor,
    pub axis_color: AnsiColor,
    pub label_color: AnsiColor,
    pub series_colors: Vec<AnsiColor>,
    pub series_legends: Vec<String>,
    pub line_ending: String,
    pub series_chars: Vec<CharSet>,
    pub x_axis_tick_count: usize,
    pub x_axis_range: Option<[f64; 2]>,
    pub x_axis_value_formatter: Option<Box<dyn Fn(f64) -> String>>,
    pub y_axis_value_formatter: Option<Box<dyn Fn(f64) -> String>>,
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
        }
    }
}

impl Config {
    /// Sets the graph width. Pass 0 to auto-scale based on data points.
    /// Width sets the graphs' width. By default, the width of the graph is
    /// determined by the number of data points. If the value given is a
    /// positive number, the data points are interpolated on the x-axis.
    /// Values = 0 reset the width to the default value.
    pub fn width(mut self, w: usize) -> Self {
        self.width = w;
        self
    }

    /// height sets the graph's height.
    pub fn height(mut self, h: usize) -> Self {
        self.height = h;
        self
    }

    /// lower_bound sets the graph's minimum value for the vertical axis. It will be ignored
    /// if the series contains a lower value.
    pub fn lower_bound(mut self, min: f64) -> Self {
        self.lower_bound = Some(min);
        self
    }

    /// upper_bound sets the graph's maximum value for the vertical axis. It will be ignored
    /// if the series contains a bigger value.
    pub fn upper_bound(mut self, max: f64) -> Self {
        self.upper_bound = Some(max);
        self
    }

    /// offset sets the graph's offset.
    pub fn offset(mut self, off: usize) -> Self {
        self.offset = off;
        self
    }

    /// precision sets the graphs precision.
    pub fn precision(mut self, p: usize) -> Self {
        self.precision = Some(p);
        self
    }

    /// caption sets the graph's caption.
    pub fn caption(mut self, c: &str) -> Self {
        self.caption = c.trim().to_string();
        self
    }

    /// caption_color sets the caption color.
    pub fn caption_color(mut self, color: AnsiColor) -> Self {
        self.caption_color = color;
        self
    }

    /// axis_color sets the axis color.
    pub fn axis_color(mut self, color: AnsiColor) -> Self {
        self.axis_color = color;
        self
    }

    /// label_color sets the axis label color.
    pub fn label_color(mut self, color: AnsiColor) -> Self {
        self.label_color = color;
        self
    }

    /// series_color sets the series colors.
    pub fn series_colors(mut self, colors: &[AnsiColor]) -> Self {
        self.series_colors = colors.to_vec();
        self
    }

    /// series_legends sets the legend text for the corresponding series.
    pub fn series_legends(mut self, text: &[&str]) -> Self {
        self.series_legends = text.iter().map(|s| s.to_string()).collect();
        self
    }

    /// line_ending sets the line ending sequence. Use "\r\n" for raw terminals
    /// (e.g., Windows terminals) or "\n" for standard Unix-style output.
    /// Defaults to "\n".
    pub fn line_ending(mut self, ending: &str) -> Self {
        self.line_ending = ending.to_string();
        self
    }

    /// series_chars sets the character sets for each series.
    /// If fewer CharSets are provided than series, Default is used for remaining series.
    pub fn series_chars(mut self, cs: &[CharSet]) -> Self {
        self.series_chars = cs.to_vec();
        self
    }

    /// x_axis_tick_count sets the number of ticks on the X-axis. Default is 5, minimum is 2.
    pub fn x_axis_tick_count(mut self, count: usize) -> Self {
        if count >= 2 {
            self.x_axis_tick_count = count;
        }
        self
    }

    /// x_axis_range enables the X-axis and maps the given domain [min, max] onto the plot width.
    pub fn x_axis_range(mut self, min: f64, max: f64) -> Self {
        self.x_axis_range = Some([min, max]);
        self
    }

    /// x_axis_value_formatter formats values printed on the X-axis.
    pub fn x_axis_value_formatter(mut self, formatter: Box<dyn Fn(f64) -> String>) -> Self {
        self.x_axis_value_formatter = Some(formatter);
        self
    }

    /// y_axis_value_formatter formats values printed on the Y-axis.
    pub fn y_axis_value_formatter(mut self, formatter: Box<dyn Fn(f64) -> String>) -> Self {
        self.y_axis_value_formatter = Some(formatter);
        self
    }
}
