use crate::asciigraph::{Bounds, Cell};
use crate::options::{Threshold};
use crate::AnsiColor;

/// Draws a horizontal reference line at each user-specified Y value.
///
/// Each threshold is associated with a specific series via `series_index`.
/// Two rules are applied per threshold before anything is drawn:
///
/// 1. **Visibility rule** — the threshold value must fall within the min/max
///    range of its associated series. If the threshold is outside that range,
///    it is silently skipped even if it falls within the global graph range.
///    This prevents thresholds from cluttering a graph when they are
///    irrelevant to their associated series.
///
/// 2. **Color inheritance rule** — if the threshold has no explicit color
///    (i.e. its color is [`AnsiColor::DEFAULT`]), it inherits the color of
///    its associated series from `series_colors`. This gives a visual cue
///    that "this threshold belongs to that line." An explicitly set color
///    always takes priority over the inherited series color.
///
/// Only blank cells in the data area are overwritten — cells already
/// containing series characters are left untouched, so the data always
/// renders on top.
pub(crate) fn render_thresholds(
    plot: &mut Vec<Vec<Cell>>,
    data: &[Vec<f64>],
    bounds: &Bounds,
    offset: usize,
    thresholds: &[Threshold],
    series_colors: &[AnsiColor],
) {
    for t in thresholds {
        // ---------------------------------------------------------------
        // Guard 1 — series index validation.
        // If the user specified a series index that does not exist in the
        // data, skip this threshold silently rather than panicking.
        // ---------------------------------------------------------------
        if t.series_index >= data.len() {
            continue;
        }

        // ---------------------------------------------------------------
        // Visibility rule — check the threshold value against the range
        // of its associated series specifically, not the global range.
        //
        // We collect only finite values because NaN represents a gap in
        // the data and has no meaningful min/max contribution.
        // ---------------------------------------------------------------
        let series = &data[t.series_index];
        let series_finite: Vec<f64> = series.iter()
            .filter(|v| v.is_finite())
            .copied()
            .collect();

        // If the series has no finite values at all, there is nothing
        // meaningful to associate this threshold with — skip it.
        if series_finite.is_empty() {
            continue;
        }

        // Find the min and max of just this series, not the global bounds.
        // fold() is used instead of min/max because f64 does not implement
        // Ord (due to NaN), so the standard min/max iterators are not
        // available. f64::min and f64::max handle this correctly.
        let series_min = series_finite.iter()
            .copied()
            .fold(f64::INFINITY, f64::min);
        let series_max = series_finite.iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);

        // If the threshold value falls outside this series' own range,
        // it is not meaningful for that series — skip it.
        if t.value < series_min || t.value > series_max {
            continue;
        }

        // ---------------------------------------------------------------
        // Color inheritance rule.
        //
        // If the threshold has an explicit color (anything other than
        // DEFAULT), use that. Otherwise, look up the color of the
        // associated series and inherit it. If no series color is
        // configured either, fall back to DEFAULT.
        //
        // This creates a natural visual association — a threshold line
        // automatically matches the color of the series it belongs to
        // unless the user overrides it explicitly.
        // ---------------------------------------------------------------
        let effective_color = if t.color != AnsiColor::DEFAULT {
            // Explicit color always wins.
            t.color
        } else {
            // Inherit from the series color if one is configured.
            series_colors.get(t.series_index)
                .copied()
                .unwrap_or(AnsiColor::DEFAULT)
        };

        // ---------------------------------------------------------------
        // Row mapping — same formula used everywhere in the pipeline.
        // ---------------------------------------------------------------
        let scaled = (t.value * bounds.ratio).round() as isize - bounds.intmin2;

        if scaled < 0 || scaled as usize > bounds.rows {
            continue;
        }

        let row = bounds.rows - scaled as usize;

        // Fill blank cells across the data area with the threshold
        // character and effective color. Occupied cells are left untouched
        // so series arc characters always render on top.
        for col in offset..plot[row].len() {
            if plot[row][col].text == " " {
                plot[row][col].text  = t.character.to_string();
                plot[row][col].color = effective_color;
            }
        }
    }
}