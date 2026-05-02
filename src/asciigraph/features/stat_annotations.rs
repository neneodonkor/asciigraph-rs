use crate::asciigraph::{Bounds, Cell};
use crate::options::{StatAnnotations, DEFAULT_CHAR_SET};

pub(crate) fn render_stat_annotations(
    plot: &mut Vec<Vec<Cell>>,
    data: &[Vec<f64>],
    bounds: &Bounds,
    offset: usize,
    annotations: &StatAnnotations,
) {
    // Step 0 — get the series_index. Especially useful for multiple lines in a graph
    let series_idx = if annotations.series_index < data.len() {
        annotations.series_index
    } else {
        0
    };

    // Step 1 — collect finite values from the first series
    let finite: Vec<f64> = data[series_idx].iter()
        .filter(|v| v.is_finite())
        .copied()
        .collect();

    if finite.is_empty() { return; }

    // Step 2 — compute the statistics you need
    let sum: f64 = finite.iter().sum();
    let mean = sum / finite.len() as f64;

    // Median — middle value when the data is arranged in order.
    let mut sorted = finite.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = sorted.len() / 2;
    let median = if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    };

    let variance = finite.iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>() / finite.len() as f64;
    let std_dev = variance.sqrt();

    // Step 3 — build a list of (value, character, label) tuples
    // for each enabled annotation
    let mut lines_to_draw: Vec<(f64, char, &str)> = Vec::new();

    if annotations.show_min {
        lines_to_draw.push((bounds.minimum, DEFAULT_CHAR_SET.dash_horizontal, "min"))
    }

    if annotations.show_max {
        lines_to_draw.push((bounds.maximum, DEFAULT_CHAR_SET.dash_horizontal, "max"));
    }

    if annotations.show_mean {
        lines_to_draw.push((mean, DEFAULT_CHAR_SET.double_dash_horizontal, "mean"));
    }

    if annotations.show_median {
        lines_to_draw.push((median, DEFAULT_CHAR_SET.heavy_dash_horizontal, "med"));
    }

    if annotations.show_std_dev {
        lines_to_draw.push((mean + std_dev, DEFAULT_CHAR_SET.dot_horizontal, "+σ"));
        lines_to_draw.push((mean - std_dev, DEFAULT_CHAR_SET.dot_horizontal, "-σ"));
    }

    // Step 4 — draw each annotation line using the same
    // row mapping formula used everywhere else
    // Track which rows have already been claimed so we can detect
    // collisions and stack labels on the same row rather than moving them.
    let mut row_labels: std::collections::HashMap<usize, String> =
        std::collections::HashMap::new();

    for (value, character, label) in &lines_to_draw {
        let scaled = (value * bounds.ratio).round() as isize - bounds.intmin2;
        if scaled < 0 || scaled as usize > bounds.rows { continue; }
        let row = bounds.rows - scaled as usize;

        // Always draw the line character — each annotation draws its own
        // line at its own row, even if two rows are the same.
        for col in offset..plot[row].len() {
            if plot[row][col].text == " " {
                plot[row][col].text  = character.to_string();
                plot[row][col].color = annotations.color;
            }
        }

        // Build the label string for this annotation.
        let new_label = format!("  {} {:.2}", label, value);

        // If another annotation already claimed this row, append this
        // label to the existing one rather than overwriting it.
        let entry = row_labels.entry(row).or_insert_with(String::new);
        if !entry.is_empty() {
            entry.push_str(&format!(", {} {:.2}", label, value));
        } else {
            *entry = new_label;
        }
    }

    // Write all the accumulated labels into the grid in a second pass.
    // Doing this after the drawing pass ensures that labels are never
    // overwritten by a subsequent annotation's line characters.
    for (row, label_text) in &row_labels {
        if let Some(last_cell) = plot[*row].last_mut() {
            last_cell.text  = label_text.clone();
            last_cell.color = annotations.color;
        }
    }
}