// This example draws a heart curve using two y-series.
//
// Note: asciigraph does not take explicit (x, y) coordinate pairs.
// It plots each input slice as a sequence of y-values against their index.
// Because the x-values here are evenly spaced from -1.0 to 1.0, the index
// positions naturally represent the horizontal axis.
//
// The upper and lower halves of the heart are generated as two separate
// vectors and then plotted together using plot_many.
//
// The shape can be tweaked by modifying the equation function or by adding
// parameters for width, height, and roundness.

use asciigraph::{Config, plot_many};

fn equation(x: f64, sign: f64) -> f64 {
    x.powi(2).cbrt() + sign * (1.0 - x.powi(2)).sqrt()
}

fn main() {
    let resolution = 1000;

    let x_values: Vec<f64> = (-resolution..=resolution)
        .map(|x| x as f64 / resolution as f64)
        .collect();

    let y_top: Vec<f64> = x_values.iter().map(|&x| equation(x, 1.0)).collect();

    let y_bottom: Vec<f64> = x_values.iter().map(|&x| equation(x, -1.0)).collect();

    let data: Vec<&[f64]> = vec![&y_top, &y_bottom];

    let graph = plot_many(
        &data,
        Config::default()
            .height(20)
            .width(60)
            .y_axis_value_formatter(Box::new(|v: f64| format!("{v:.2}"))),
    );

    println!("{graph}");
}
