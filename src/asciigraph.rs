// The main file that plots the graph.

use crate::legend::add_legends;
use crate::options::{CharSet, Config, DEFAULT_CHAR_SET};
use crate::utils::{calculate_height, interpolate_array, min_max_float64_slice};
use crate::{utils, AnsiColor};

#[derive(Clone)]
struct Cell {
    text: String,
    color: AnsiColor
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            text: " ".to_string(),
            color: AnsiColor::DEFAULT,
        }
    }
}

/// Plot returns ascii graph for a series.
pub fn plot(series: &[f64], config: Config) -> String {
    plot_many(&mut [series], config)
}

/// get_char_set returns the CharSet for a given series index, falling back to DEFAULT_CHAR_SET.
pub(crate) fn get_char_set(config: &Config, series_index: usize) -> CharSet {
    if series_index < config.series_chars.len() {
        let mut char_set = config.series_chars[series_index].clone();

        if char_set.horizontal.is_empty() {
            char_set.horizontal = DEFAULT_CHAR_SET.horizontal;
        }

        if char_set.vertical_line.is_empty() {
            char_set.vertical_line = DEFAULT_CHAR_SET.vertical_line;
        }

        if char_set.arc_down_right.is_empty() {
            char_set.arc_down_right = DEFAULT_CHAR_SET.arc_down_right
        }

        if char_set.arc_down_left.is_empty() {
            char_set.arc_down_left = DEFAULT_CHAR_SET.arc_down_left;
        }

        if char_set.arc_up_right.is_empty() {
            char_set.arc_up_right = DEFAULT_CHAR_SET.arc_up_right;
        }

        if char_set.arc_up_left.is_empty() {
            char_set.arc_up_left = DEFAULT_CHAR_SET.arc_up_left;
        }

        if char_set.end_cap.is_empty() {
            char_set.end_cap = DEFAULT_CHAR_SET.end_cap;
        }

        if char_set.start_cap.is_empty() {
            char_set.start_cap = DEFAULT_CHAR_SET.start_cap;
        }

        if char_set.up_right.is_empty() {
            char_set.up_right = DEFAULT_CHAR_SET.up_right;
        }

        if char_set.down_horizontal.is_empty() {
            char_set.down_horizontal = DEFAULT_CHAR_SET.down_horizontal;
        }

        return char_set;
    }

    DEFAULT_CHAR_SET
}

//0  -----------------------------------------------------------------------------------------------
pub fn plot_many(data: &[&[f64]], config: Config) -> String {
    let mut config = config;

    if config.offset == 0 {
        config.offset = 3;
    }

    if config.line_ending.is_empty() {
        config.line_ending = "\n".to_string();
    }

    // deep copy on input data — from here on we work with owned data
    let mut data: Vec<Vec<f64>> = data.iter().map(|s| s.to_vec()).collect();

    // now len_max can use the owned data
    let mut len_max: usize = 0;

    // Might be for i in 0..data.len() because in Go it is for i := range data
    for series in data.iter() {
        len_max = len_max.max(series.len());

        // This is the same as this:
        /*let l = data[i].len();
        if l > len_max {
            len_max = l;
        }*/
    }

    // padding and interpolation
    if config.width > 0 {
        for i in 0..data.len() {
            while data[i].len() < len_max {
                data[i].push(f64::NAN);
            }

            data[i] = interpolate_array(&data[i], config.width as u32);
        }

        len_max = config.width;
    }

    // 1 -------------------------------------------------------------------------------------------

    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;

    for i in 0..data.len() {
        let values = min_max_float64_slice(&data[i]);

        match values {
            Some((min_value, max_value)) => {
                if min_value < minimum {
                    minimum = min_value;
                }

                if max_value > maximum {
                    maximum = max_value;
                }
            },
            None => println!("Values were not provided"),
        }
    }

    if let Some(lb) = config.lower_bound {
        if lb < minimum { minimum = lb; }
    }

    if let Some(ub) = config.upper_bound {
        if ub > maximum { maximum = ub; }
    }

    let interval = (maximum - minimum).abs();

    if config.height <= 0 {
        config.height = calculate_height(interval)
    }

    if config.offset <= 0 {
        config.offset = 3;
    }

    let ratio: f64;
    if interval != 0.0 {
        ratio = config.height as f64 / interval;
    } else {
        ratio = 1.0;
    }

    let min2 = utils::round(minimum * ratio);
    let max2 = utils::round(maximum * ratio);
    let intmin2 = min2.round() as isize;
    let intmax2 = max2.round() as isize;

    let rows = (intmax2 - intmin2).unsigned_abs();
    let width = len_max + config.offset;

    // 2 -------------------------------------------------------------------------------------------

    // initialize empty 2D grid.
    let mut plot: Vec<Vec<Cell>> = vec![vec![Cell::default(); width]; rows + 1];

    // Default precision to maintain backwards compatibility.
    let mut precision: usize = config.precision.unwrap_or(2);

    // To find number of zeros after decimal
    let mut log_maximum = maximum.abs().max(minimum.abs()).log10();

    if minimum == 0.0 && maximum == 0.0 {
        log_maximum = f64::from(-1);
    }

    if log_maximum < 0.0 {
        // Negative log
        // log_maximum.fract() is the same as log_maximum % 1.0 (modulus).
        if log_maximum.fract() != 0.0 {
            precision += log_maximum.abs() as usize;
        } else {
            precision += (log_maximum.abs() - 1.0) as usize;
        }
    } else if log_maximum > 2.0 && config.precision.is_none() {
        precision = 0;
    }

    // 3 -------------------------------------------------------------------------------------------

    let mut max_num_length = format!("{:.1$}", maximum, precision).chars().count();
    let min_num_length = format!("{:.precision$}", minimum, precision = precision).chars()
        .count();
    let mut magnitudes: Vec<f64> = Vec::with_capacity(rows + 1);

    if config.y_axis_value_formatter.is_some() {
        max_num_length = 0;
    }

    // calculate Y-axis values and the width when formatted using the y_axis_value_formatter
    let y = intmin2;
    for y in y..intmax2 + 1 {
        let magnitude: f64;

        if rows > 0 && interval > 0.0 {
            magnitude = maximum - (((y - intmin2) as f64 * interval) / rows as f64);
        } else if interval == 0.0 {
            magnitude = minimum;
        } else {
            magnitude = y as f64;
        }

        magnitudes.push(magnitude);

        if let Some(formatter) = &config.y_axis_value_formatter {
            let result = formatter(magnitude);
            let l = result.chars().count();
            if l > max_num_length {
                max_num_length = l;
            }
        }
    }

    let max_width: usize;
    if config.y_axis_value_formatter.is_some() {
        max_width = max_num_length;
    } else {
        max_width = (max_num_length as f64).max(min_num_length as f64) as usize;
    }

    let left_pad = config.offset + max_width;

    // 4 -------------------------------------------------------------------------------------------

    // axis and labels reusing the previously calculated values.
    for (w, magnitude) in magnitudes.iter().enumerate() {
        let mut label = String::new();

        if config.y_axis_value_formatter.is_none() {
            // > means right-aligned as this is the default in Go string formatting.
            label = format!("{:>width$.prec$}", magnitude, width = max_width, prec = precision);
        } else {
            if let Some(formatter) = &config.y_axis_value_formatter {
                let val = formatter(*magnitude);
                label = " ".repeat((max_width + 1) - (val.chars().count()));
            }
        }

        let float_value = (config.offset as f64) - (label.chars().count() as f64);
        let h = float_value.max(0.0) as usize;

        plot[w][h].text = label;
        plot[w][h].color = config.label_color;
        plot[w][config.offset - 1].text = "┤".to_string();
        plot[w][config.offset - 1].color = config.axis_color;
    }

    for i in 0..data.len() {
        let series = &data[i];
        let mut color = AnsiColor::DEFAULT;

        if i < config.series_colors.len() {
            color = config.series_colors[i];
        }

        // Get the character set for this series.
        let char_set = get_char_set(&config, i);
        let (mut y0, mut y1): (usize, usize);

        if !series[0].is_nan() {
            y0 = ((series[0] * ratio).round() - min2) as usize;
            plot[rows - y0][config.offset - 1].text = "┼".to_string(); // first value
            plot[rows - y0][config.offset - 1].color = config.axis_color;
        }

        // Not included in the Go code, but I added this guard to avoid the "empty vector" trap.
        // since in the loop, we have the condition: series.len() - 1, which could result in -1
        if series.len() > 1 {
            for x in 0..series.len() - 1 {
                let d0 = series[x];
                let d1 = series[x+1];

                if d0.is_nan() && d1.is_nan() {
                    continue;
                }

                if !d0.is_nan() && d1.is_nan() {
                    y0 = ((d0 * ratio).round() - intmin2 as f64) as usize;
                    plot[rows - y0][x + config.offset].text = char_set.end_cap.to_string();
                    plot[rows - y0][x + config.offset].color = color;
                    continue;
                }

                if d0.is_nan() && !d1.is_nan() {
                    y1 = ((d1 * ratio).round() - intmin2 as f64) as usize;
                    plot[rows - y1][x + config.offset].text = char_set.start_cap.to_string();
                    plot[rows - y1][x + config.offset].color = color;
                    continue;
                }

                y0 = ((d0 * ratio).round() - intmin2 as f64) as usize;
                y1 = ((d1 * ratio).round() - intmin2 as f64) as usize;

                if y0 == y1 {
                    plot[rows - y0][x + config.offset].text = char_set.horizontal.to_string();
                } else {
                    if y0 > y1 {
                        plot[rows - y1][x + config.offset].text = char_set.arc_up_right.to_string();
                        plot[rows - y0][x + config.offset].text = char_set.arc_down_left.to_string();
                    } else {
                        plot[rows - y1][x + config.offset].text = char_set.arc_down_right.to_string();
                        plot[rows - y0][x + config.offset].text = char_set.arc_up_left.to_string();
                    }

                    let start = (y0 as f64).min(y1 as f64) as usize + 1;
                    let end = (y0 as f64).max(y1 as f64) as usize;
                    let y = start;
                    for y in y..end {
                        plot[rows - y][x + config.offset].text = char_set.vertical_line.to_string();
                    }
                }

                let start = (y0 as f64).min(y1 as f64) as usize;
                let end = (y0 as f64).max(y1 as f64) as usize;
                let y = start;
                for y in y..=end {
                    plot[rows - y][x + config.offset].color = color;
                }
            }
        }
    }

    // 5 -------------------------------------------------------------------------------------------

    // join colums
    let mut lines = String::new();

    for (h, horizontal) in plot.iter().enumerate() {
        if h != 0 {
            lines.push_str(&config.line_ending);
        }

        // remove trailing spaces
        let mut last_char_index = 0;

        for i in (0..width).rev() {
            if horizontal[i].text != " " {
                last_char_index = i;
                break;
            }
        }

        let mut c = AnsiColor::DEFAULT;

        for v in horizontal[..=last_char_index].iter() {
            if v.color != c {
                c = v.color;
                lines.push_str(c.to_string().as_str());
            }

            lines.push_str(v.text.as_str());
        }

        if c != AnsiColor::DEFAULT {
            lines.push_str(AnsiColor::DEFAULT.to_string().as_str());
        }
    }

    // add x-axis if configured
    if config.x_axis_range.is_some() {
        add_x_axis(&mut lines, &config, len_max, left_pad);
    }

    // add caption if not empty
    if !config.caption.is_empty() {
        lines.push_str(&config.line_ending);
        lines.push_str(&" ".repeat(left_pad));

        if config.caption.len() < len_max {
            lines.push_str(&" ".repeat((len_max - config.caption.len())/2));
        }

        if config.caption_color != AnsiColor::DEFAULT {
            lines.push_str(config.caption_color.to_string().as_str());
        }

        lines.push_str(config.caption.as_str());

        if config.caption_color != AnsiColor::DEFAULT {
            lines.push_str(AnsiColor::DEFAULT.to_string().as_str())
        }
    }

    if config.series_legends.len() > 0 {
        add_legends(&mut lines, &config, len_max, left_pad);
    }

    lines.to_string()
}

//-------------------------------------------------------------------------------------------------

// default_x_axis_formatter will be implemented in the function add_x_axis.

// add_x_axis appends an X-axis line and tick labels below the plot body.
fn add_x_axis(lines: &mut String, config: &Config, len_max: usize, left_pad: usize) {
    let default_x_axis_formatter = |v: f64| format!("{}", v);

    if len_max <= 0 {
        return;
    }

    let x_min = config.x_axis_range.unwrap()[0];
    let x_max = config.x_axis_range.unwrap()[1];
    let mut tick_count = config.x_axis_tick_count;

    if len_max == 1 {
        tick_count = 1;
    } else if tick_count < 2 {
        tick_count = 5;
    }

    if tick_count > len_max {
        tick_count = len_max;
    }

    let decimal_formatter: fn(f64) -> String = |v: f64| format!("{:.2}", v);
    let default_fn: fn(f64) -> String = default_x_axis_formatter;
    let mut formatter = config.x_axis_value_formatter.as_ref().map(|f| f.as_ref());

    // compute tick column positions and values.
    struct Tick {
        col: usize,
        value: f64,
        label: String
    }

    let mut ticks: Vec<Tick> = Vec::with_capacity(tick_count);

    for i in 0..tick_count {
        let col = if tick_count == 1 {
            0
        } else {
            (((len_max - 1) as f64 * i as f64) / (tick_count - 1) as f64).round() as usize
        };

        let value = if tick_count == 1 {
            x_min
        } else {
            x_min + (i as f64 / (tick_count - 1) as f64) * (x_max - x_min)
        };

        ticks.push(Tick { col, value, label: String::new() });
    }

    // select formatter: when using default, auto-detect precision based on visible ticks
    if formatter.is_none() {
        // simulate overlap with labels to find visible ticks with fractional values
        let mut has_decimal = false;
        let mut last_end: isize = -1;

        for i in 0..ticks.len() {
            let label = default_x_axis_formatter(ticks[i].value);
            let label_len = label.chars().count();
            let start_col = left_pad + ticks[i].col - (label_len / 2);

            if start_col as isize > last_end {
                if ticks[i].value != ticks[i].value.floor() {
                    has_decimal = true;
                    break;
                }

                last_end = start_col as isize + label_len as isize;
            }
        }

        if has_decimal {
            formatter = Some(&decimal_formatter);
        } else {
            formatter = Some(&default_fn);
        }
    }

    // format labels
    for i in 0..ticks.len() {
        ticks[i].label = formatter.unwrap()(ticks[i].value);
    }

    // axis line: leftPad-1 spaces + └ + ─/┬ characters
    let total_width = left_pad + len_max;
    let mut axis_line: Vec<char> = vec![' '; total_width];

    axis_line[left_pad - 1] = DEFAULT_CHAR_SET.up_right.chars().next().unwrap();

    for i in 0..len_max {
        axis_line[left_pad + i] = DEFAULT_CHAR_SET.horizontal.chars().next().unwrap();
    }

    for tk in &ticks {
        axis_line[left_pad + tk.col] = DEFAULT_CHAR_SET.down_horizontal.chars().next().unwrap();
    }

    // write axis line with colors
    lines.push_str(config.line_ending.as_str());
    let text_stream = axis_line.iter().collect::<String>();
    let axis_str = text_stream.trim_end();

    if config.axis_color != AnsiColor::DEFAULT {
        lines.push_str(config.axis_color.to_string().as_str());
    }

    lines.push_str(axis_str);

    if config.axis_color != AnsiColor::DEFAULT {
        lines.push_str(AnsiColor::DEFAULT.to_string().as_str());
    }

    // label line: place each label centered on its tick column.
    let mut max_right_extent = total_width;

    for tk in &ticks {
        let label_len = tk.label.chars().count();
        let end_col = left_pad + tk.col + (label_len - (label_len / 2));

        if end_col > max_right_extent {
            max_right_extent = end_col;
        }
    }

    let mut label_line: Vec<char> = vec![' '; max_right_extent];

    for i in 0..label_line.len() {
        label_line[i] = ' ';
    }

    let mut last_end: isize = -1; // tracks the rightmost column used by the previous label.

    for tk in &ticks {
        let label_runes: Vec<char> = tk.label.chars().collect();
        let label_len = label_runes.len();

        // center the label on the tick column
        let mut start_col = (left_pad + tk.col) as isize - (label_len / 2) as isize;

        if start_col < 0 {
            start_col = 0;
        }

        // skip if this label would overlap the previous one (need 1-space gap)
        if start_col <= last_end {
            continue
        }

        for (j, r) in label_runes.iter().enumerate() {
            let pos = start_col + j as isize;

            if pos < label_line.len() as isize {
                label_line[pos as usize] = *r;
            }
        }

        last_end = start_col + label_len as isize;
    }

    // trim and write label line
    let text = label_line.iter().collect::<String>();
    let label_str = text.trim_end();

    if !label_str.is_empty() {
        lines.push_str(config.line_ending.as_str());

        if config.label_color != AnsiColor::DEFAULT {
            lines.push_str(config.label_color.to_string().as_str());
        }

        lines.push_str(label_str);

        if config.label_color != AnsiColor::DEFAULT {
            lines.push_str(AnsiColor::DEFAULT.to_string().as_str());
        }
    }
}