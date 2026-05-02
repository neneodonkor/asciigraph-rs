use asciigraph::{plot, Config, StatAnnotations, AnsiColor};

fn main() {
    let data = vec![
        3.0, 1.0, 5.0, 2.0, 8.0, 4.0, 7.0, 2.0, 6.0, 3.0,
        9.0, 4.0, 6.0, 2.0, 7.0, 3.0, 8.0, 1.0, 5.0, 3.0,
    ];

    let graph = plot(
        &data,
        Config::default()
            .stat_annotations(StatAnnotations::with_color(AnsiColor::RED)),
    );

    println!("{}", graph);
}