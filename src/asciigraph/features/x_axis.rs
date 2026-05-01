use crate::AnsiColor;
use crate::options::{Config, DEFAULT_CHAR_SET};

// ---------------------------------------------------------------------------
// X-axis rendering
// ---------------------------------------------------------------------------

/// Appends an X-axis line and tick labels below the plot body.
///
/// Computes tick positions evenly distributed across the data width, formats
/// their labels using the configured or auto-detected formatter, and renders
/// two lines: the axis line (with `└`, `─`, and `┬` characters) and the label
/// line (with each label centered on its tick column). Labels that would
/// overlap a previous label are skipped to avoid visual clutter.
pub(crate) fn add_x_axis(lines: &mut String, config: &Config, len_max: usize, left_pad: usize) {
    let default_x_axis_formatter = |v: f64| format!("{}", v);

    if len_max == 0 {
        return;
    }

    let x_min = config.x_axis_range.unwrap()[0];
    let x_max = config.x_axis_range.unwrap()[1];
    let mut tick_count = config.x_axis_tick_count;

    if len_max == 1 {
        tick_count = 1;
    } else if tick_count < 2 {
        let min_label = default_x_axis_formatter(x_min);
        let max_label = default_x_axis_formatter(x_max);
        let avg_label_width = (min_label.chars().count() + max_label.chars().count()) / 2 + 1;
        tick_count = (len_max / avg_label_width).max(2);
    }

    if tick_count > len_max {
        tick_count = len_max;
    }

    let decimal_formatter: fn(f64) -> String = |v: f64| format!("{:.2}", v);
    let default_fn: fn(f64) -> String = default_x_axis_formatter;
    let mut formatter = config.x_axis_value_formatter.as_ref().map(|f| f.as_ref());

    struct Tick {
        col: usize,
        value: f64,
        label: String,
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

    if formatter.is_none() {
        let mut has_decimal = false;
        let mut last_end: isize = -1;

        for tk in &ticks {
            let label = default_x_axis_formatter(tk.value);
            let label_len = label.chars().count();
            let start_col = left_pad + tk.col - (label_len / 2);

            if start_col as isize > last_end {
                if tk.value != tk.value.floor() {
                    has_decimal = true;
                    break;
                }
                last_end = start_col as isize + label_len as isize;
            }
        }

        formatter = if has_decimal {
            Some(&decimal_formatter as &dyn Fn(f64) -> String)
        } else {
            Some(&default_fn as &dyn Fn(f64) -> String)
        };
    }

    for tk in ticks.iter_mut() {
        tk.label = formatter.unwrap()(tk.value);
    }

    let total_width = left_pad + len_max;
    let mut axis_line: Vec<char> = vec![' '; total_width];

    axis_line[left_pad - 1] = DEFAULT_CHAR_SET.up_right;
    for i in 0..len_max {
        axis_line[left_pad + i] = DEFAULT_CHAR_SET.horizontal;
    }
    for tk in &ticks {
        axis_line[left_pad + tk.col] = DEFAULT_CHAR_SET.down_horizontal;
    }

    lines.push_str(&config.line_ending);
    let axis_str: String = axis_line.iter().collect::<String>();
    let axis_str = axis_str.trim_end();

    if config.axis_color != AnsiColor::DEFAULT {
        lines.push_str(&config.axis_color.to_string());
    }
    lines.push_str(axis_str);
    if config.axis_color != AnsiColor::DEFAULT {
        lines.push_str(&AnsiColor::DEFAULT.to_string());
    }

    let mut max_right_extent = total_width;
    for tk in &ticks {
        let label_len = tk.label.chars().count();
        let end_col = left_pad + tk.col + (label_len - (label_len / 2));
        if end_col > max_right_extent {
            max_right_extent = end_col;
        }
    }

    let mut label_line: Vec<char> = vec![' '; max_right_extent];
    let mut last_end: isize = -1;

    for tk in &ticks {
        let label_runes: Vec<char> = tk.label.chars().collect();
        let label_len = label_runes.len();
        let mut start_col = (left_pad + tk.col) as isize - (label_len / 2) as isize;

        if start_col < 0 { start_col = 0; }
        if start_col <= last_end { continue; }

        for (j, r) in label_runes.iter().enumerate() {
            let pos = start_col + j as isize;
            if pos < label_line.len() as isize {
                label_line[pos as usize] = *r;
            }
        }

        last_end = start_col + label_len as isize;
    }

    let label_str: String = label_line.iter().collect::<String>();
    let label_str = label_str.trim_end();

    if !label_str.is_empty() {
        lines.push_str(&config.line_ending);
        if config.label_color != AnsiColor::DEFAULT {
            lines.push_str(&config.label_color.to_string());
        }
        lines.push_str(label_str);
        if config.label_color != AnsiColor::DEFAULT {
            lines.push_str(&AnsiColor::DEFAULT.to_string());
        }
    }
}