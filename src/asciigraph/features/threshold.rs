use crate::asciigraph::{Bounds, Cell};
use crate::options::Threshold;

// ---------------------------------------------------------------------------
// Threshold rendering
// ---------------------------------------------------------------------------

/// Draws a horizontal reference line at each user-specified Y value.
///
/// Each threshold value is mapped to a grid row using the general row-mapping
/// formula: `(value * ratio).round() as isize - intmin2`. Thresholds that
/// fall outside the visible range are silently skipped via a `continue`.
///
/// Only blank cells in the data area are overwritten — cells already
/// containing series characters, zero-line characters, or other content are
/// left untouched. This function is called before [`render_series`] so that
/// series arc characters always appear on top of threshold lines.
pub(crate) fn render_thresholds(
    plot: &mut Vec<Vec<Cell>>,
    bounds: &Bounds,
    offset: usize,
    thresholds: &[Threshold],
) {
    for t in thresholds {
        let scaled = (t.value * bounds.ratio).round() as isize - bounds.intmin2;

        if scaled < 0 || scaled as usize > bounds.rows {
            continue;
        }

        let row = bounds.rows - scaled as usize;

        for col in offset..plot[row].len() {
            if plot[row][col].text == " " {
                plot[row][col].text  = t.character.to_string();
                plot[row][col].color = t.color;
            }
        }
    }
}