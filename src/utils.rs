// Utility functions

pub(crate) fn min_max_float64_slice(v: &[f64]) -> Option<(f64, f64)>  {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    if v.is_empty() {
       return None;
    }

    for e in v {
        if e < &min {
            min = *e;
        }

        if e > &max {
            max = *e;
        }
    }

    Some((min, max))
}

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
    //let whole = input.trunc(); For whole number
    let rounded: f64;

    if decimal >= 0.5 {
        rounded = input.ceil();
    } else {
        rounded = input.floor();
    }

    rounded * sign
}

pub(crate) fn linear_interpolate(before: f64, after: f64, at_point: f64) -> f64 {
    before + (after - before) * at_point
}

pub(crate) fn interpolate_array(data: &[f64], fit_count: u32) -> Vec<f64> {
    let mut interpolated_data = Vec::new();

    let spring_factor = (data.len() - 1) as f64 / (fit_count - 1) as f64;
    interpolated_data.push( data[0]);

    for i in 1..fit_count - 1 {
        let spring = f64::from(i) * spring_factor;
        let before = spring.floor();
        let after = spring.ceil();
        let at_point = spring - before;
        interpolated_data.push(linear_interpolate(data[before as usize], data[after as usize], at_point));
    }

    interpolated_data.push(data[data.len()-1]);

    interpolated_data
}

// GO CODE YET TO BE PORTED
// clear terminal screen
/*var Clear func()

func init() {
    platform := runtime.GOOS

    if platform == "windows" {
        Clear = func() {
            cmd := exec.Command("cmd", "/c", "cls")
            cmd.Stdout = os.Stdout
            if err := cmd.Run(); err != nil {
              log.Fatal(err)
            }
        }
    } else {
        Clear = func() {
            fmt.Print("\033[2J\033[H")
        }
    }
}*/

pub(crate) fn calculate_height(interval: f64) -> u32 {
    if interval >= 1.0 {
        return interval as u32;
    }

    let scale_factor = 10f64.powf(interval.log10().floor());
    let scaled_delta = interval / scale_factor;

    if scaled_delta < 2.0 {
        return scaled_delta.ceil() as u32;
    }

    scaled_delta.floor() as u32
}