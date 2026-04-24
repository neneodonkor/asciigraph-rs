use asciigraph::{plot_many, Config, AnsiColor};

fn main() {
    // Build 6 series of concentric semi-circles.
    // Each series i has a radius of (40 - i), giving progressively smaller arcs.
    let data: Vec<Vec<f64>> = (0..6)
        .map(|i| {
            (-40..=40)
                .map(|x| {
                    let r = 40 - i;
                    // Only compute the arc where x is within the radius.
                    // Outside that range the value is NaN — which asciigraph renders as a gap.
                    if x >= -r && x <= r {
                        let r = r as f64;
                        let x = x as f64;
                        // Semi-circle formula: y = sqrt(r² - x²) / 2
                        (r * r - x * x).sqrt() / 2.0
                    } else {
                        f64::NAN
                    }
                })
                .collect()
        })
        .collect();

    // Convert each owned Vec<f64> into a &[f64] slice reference for plot_many
    let refs: Vec<&[f64]> = data.iter().map(|s| s.as_slice()).collect();

    let graph = plot_many(
        &refs,
        Config::default()
            .precision(0)
            .series_colors(&[
                AnsiColor::RED,
                AnsiColor::ORANGE,
                AnsiColor::YELLOW,
                AnsiColor::GREEN,
                AnsiColor::BLUE,
                AnsiColor::PURPLE,
            ]),
    );

    println!("{}", graph);
}