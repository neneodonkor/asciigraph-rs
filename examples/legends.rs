use asciigraph::{plot_many, Config, AnsiColor};

fn main() {
    let data: Vec<Vec<f64>> = (0..3)
        .map(|i| {
            (-12..=12)
                .map(|x| {
                    let r = 12 - i;
                    if x >= -r && x <= r {
                        let r = r as f64;
                        let x = x as f64;
                        (r * r - x * x).sqrt() / 2.0
                    } else {
                        f64::NAN
                    }
                })
                .collect()
        })
        .collect();

    let refs: Vec<&[f64]> = data.iter().map(|s| s.as_slice()).collect();

    let graph = plot_many(
        &refs,
        Config::default()
            .precision(0)
            .series_colors(&[AnsiColor::RED, AnsiColor::GREEN, AnsiColor::BLUE])
            .series_legends(&["Red", "Green", "Blue"])
            .caption("Series with legends"),
    );

    println!("{}", graph);
}