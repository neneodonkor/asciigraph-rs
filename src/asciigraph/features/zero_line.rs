use crate::asciigraph::{Bounds, Cell};
use crate::options::ZeroLine;

// ---------------------------------------------------------------------------
// Zero line rendering
// ---------------------------------------------------------------------------

/// Draws a horizontal reference line across the data area at Y = 0.0.
///
/// The value `0.0` is mapped to a grid row using the same formula used for
/// every other value. Because the input is `0.0`, the formula simplifies to
/// `rows - (-intmin2)`. If the result falls outside the visible grid (i.e.
/// all data is positive or all data is negative), the function returns
/// without drawing anything.
///
/// Only blank cells in the data area are overwritten — cells already
/// containing series characters or other content are left untouched.
/// This function is called before [`render_series`] so that series arc
/// characters always appear on top of the zero line.
pub(crate) fn render_zero_line(
    plot: &mut Vec<Vec<Cell>>,
    bounds: &Bounds,
    offset: usize,
    zero_line: ZeroLine,
) {
    let zero_scaled = -bounds.intmin2;

    if zero_scaled < 0 || zero_scaled as usize > bounds.rows {
        return;
    }

    let row = bounds.rows - zero_scaled as usize;

    for col in offset..plot[row].len() {
        if plot[row][col].text == " " {
            plot[row][col].text  = zero_line.character.to_string();
            plot[row][col].color = zero_line.color;
        }
    }
}