// Utility functions

/// Returns the minimum and maximum finite values in a slice.
///
/// Iterates over every element and tracks the running min and max. Returns
/// `None` if the slice is empty. Note that `NaN` values are not explicitly
/// filtered — if the slice contains only `NaN`, the returned min and max will
/// be `INFINITY` and `NEG_INFINITY` respectively, which the caller should
/// guard against.
pub(crate) fn min_max_float64_slice(v: &[f64]) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    if v.is_empty() {
        return None;
    }

    for e in v {
        if e < &min { min = *e; }
        if e > &max { max = *e; }
    }

    Some((min, max))
}

/// Rounds a floating-point number to the nearest integer using standard
/// rounding (0.5 rounds up).
///
/// Handles negative numbers correctly by preserving the sign before rounding
/// and reapplying it afterward. Returns `NaN` unchanged.
pub(crate) fn round(mut input: f64) -> f64 {
    if input.is_nan() {
        return f64::NAN;
    }

    let mut sign = 1.0;
    if input < 0.0 {
        sign = -1.0;
        input *= -1.0;
    }

    let decimal = input.fract();
    let rounded = if decimal >= 0.5 { input.ceil() } else { input.floor() };

    rounded * sign
}

/// Linearly interpolates between two values.
///
/// Returns the value at `at_point` along the segment from `before` to `after`,
/// where `at_point = 0.0` returns `before` and `at_point = 1.0` returns `after`.
pub(crate) fn linear_interpolate(before: f64, after: f64, at_point: f64) -> f64 {
    before + (after - before) * at_point
}

/// Resamples a data slice to exactly `fit_count` points using linear
/// interpolation.
///
/// The first and last points of the input are always preserved as the first
/// and last points of the output. Interior points are computed by linearly
/// interpolating between the two nearest input values. This is used to scale
/// a series to a user-specified width before rendering.
///
/// # Panics
///
/// Panics if `fit_count` is less than 2 or if `data` is empty, as the
/// spring factor calculation would produce a division by zero.
pub(crate) fn interpolate_array(data: &[f64], fit_count: u32) -> Vec<f64> {
    let mut interpolated_data = Vec::new();

    let spring_factor = (data.len() - 1) as f64 / (fit_count - 1) as f64;
    interpolated_data.push(data[0]);

    for i in 1..fit_count - 1 {
        let spring = f64::from(i) * spring_factor;
        let before = spring.floor();
        let after = spring.ceil();
        let at_point = spring - before;
        interpolated_data.push(linear_interpolate(
            data[before as usize],
            data[after as usize],
            at_point,
        ));
    }

    interpolated_data.push(data[data.len() - 1]);

    interpolated_data
}

/// Computes an appropriate graph height in rows for a given data interval.
///
/// For intervals of 1.0 or greater, the height is simply the interval
/// truncated to an integer. For smaller intervals, a scale factor is derived
/// from the order of magnitude of the interval so that the graph height
/// remains readable regardless of how small the data range is.
pub(crate) fn calculate_height(interval: f64) -> usize {
    if interval >= 1.0 {
        return interval as usize;
    }

    let scale_factor = 10f64.powf(interval.log10().floor());
    let scaled_delta = interval / scale_factor;

    if scaled_delta < 2.0 {
        return scaled_delta.ceil() as usize;
    }

    scaled_delta.floor() as usize
}

/// Computes a simple moving average over a data series.
///
/// Each output point is the mean of the finite values within a window of
/// `window` points centered on that index. At the edges of the series where
/// a full window does not fit, a partial window is used instead of padding
/// with NaN, producing a smoother result at the boundaries.
///
/// NaN values within the window are excluded from the average. If a window
/// contains no finite values at all, the corresponding output point is NaN.
///
/// A window of 0 or 1 returns a copy of the input unchanged.
///
/// # Example
///
/// ```rust
/// use asciigraph::utils::moving_average;
///
/// let data = vec![1.0, 3.0, 5.0, 3.0, 1.0];
/// let smoothed = moving_average(&data, 3);
/// ```
pub fn moving_average(data: &[f64], window: usize) -> Vec<f64> {
    if window <= 1 || data.is_empty() {
        return data.to_vec();
    }

    let n = data.len();
    let half = window / 2;
    let mut result = Vec::with_capacity(n);

    for i in 0..n {
        let start = i.saturating_sub(half);
        let end = (i + half + 1).min(n);

        let mut sum = 0.0;
        let mut count = 0usize;

        for &v in &data[start..end] {
            if v.is_finite() {
                sum += v;
                count += 1;
            }
        }

        result.push(if count > 0 { sum / count as f64 } else { f64::NAN });
    }

    result
}