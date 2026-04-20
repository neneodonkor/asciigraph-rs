// Handle legend items

use crate::color::AnsiColor;
use crate::options::Config;

pub(crate) fn create_legend_item(text: &str, color: AnsiColor) -> (String, usize) {
    let t = format!("{}■{} {}", color, AnsiColor::DEFAULT, text);
    let l = text.chars().count() + 2;

    (t, l)
}

pub(crate) fn add_legends(lines: &mut String, config: &Config, len_max: usize, left_pad: usize) {
    lines.push_str(&config.line_ending);
    lines.push_str(&config.line_ending);
    let padding = " ".repeat(left_pad);
    lines.push_str(padding.as_str());

    let mut legends_text: String = String::new();
    let mut legends_text_len: usize = 0;
    let right_pad: usize = 3;

    for (i, text) in config.series_legends.iter().enumerate() {
        let mut color = AnsiColor::DEFAULT;
        if i < config.series_colors.len() {
            color = config.series_colors[i];
        }

        let (item, item_len) = create_legend_item(text, color);
        legends_text.push_str(&item);
        legends_text_len += item_len;

        if i < config.series_legends.len() - 1 {
            let txt = " ".repeat(right_pad);
            legends_text.push_str(&txt);
            legends_text_len += right_pad;
        }
    }

    if legends_text_len < len_max {
        let txt = " ".repeat((len_max - legends_text_len)/2);
        lines.push_str(&txt);
    }

    lines.push_str(&legends_text)
}