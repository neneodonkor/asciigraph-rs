use asciigraph::{plot, Config, Threshold, AnsiColor, plot_many};

fn main() {
    let data = vec![
        60.0, 65.0, 72.0, 78.0, 85.0, 91.0, 88.0, 76.0, 70.0, 64.0,
        68.0, 75.0, 82.0, 89.0, 94.0, 87.0, 79.0, 71.0, 66.0, 60.0,
    ];

    let graph = plot(
        &data,
        Config::default()
            .threshold(Threshold::with_color(80.0, AnsiColor::YELLOW))
            .threshold(Threshold::with_color(90.0, AnsiColor::RED)),
    );

    println!("{}", graph);

    // Output:
    // 94.00 ┤             ╭╮
    // 93.00 ┤             ││
    // 92.00 ┤             ││
    // 91.00 ┤    ╭╮       ││
    // 90.00 ┤╌╌╌╌││╌╌╌╌╌╌╌││╌╌╌╌╌
    // 89.00 ┤    ││      ╭╯│
    // 88.00 ┤    │╰╮     │ │
    // 87.00 ┤    │ │     │ ╰╮
    // 86.00 ┤    │ │     │  │
    // 85.00 ┤   ╭╯ │     │  │
    // 84.00 ┤   │  │     │  │
    // 83.00 ┤   │  │     │  │
    // 82.00 ┤   │  │    ╭╯  │
    // 81.00 ┤   │  │    │   │
    // 80.00 ┤╌╌╌│╌╌│╌╌╌╌│╌╌╌│╌╌╌╌
    // 79.00 ┤   │  │    │   ╰╮
    // 78.00 ┤  ╭╯  │    │    │
    // 77.00 ┤  │   │    │    │
    // 76.00 ┤  │   ╰╮   │    │
    // 75.00 ┤  │    │  ╭╯    │
    // 74.00 ┤  │    │  │     │
    // 73.00 ┤  │    │  │     │
    // 72.00 ┤ ╭╯    │  │     │
    // 71.00 ┤ │     │  │     ╰╮
    // 70.00 ┤ │     ╰╮ │      │
    // 69.00 ┤ │      │ │      │
    // 68.00 ┤ │      │╭╯      │
    // 67.00 ┤ │      ││       │
    // 66.00 ┤ │      ││       ╰╮
    // 65.00 ┤╭╯      ││        │
    // 64.00 ┤│       ╰╯        │
    // 63.00 ┤│                 │
    // 62.00 ┤│                 │
    // 61.00 ┤│                 │
    // 60.00 ┼╯                 ╰

    threshold_multi_series();

    //Output:
    // 91.00 ┤   ╭╮
    // 90.00 ┤   ││
    // 89.00 ┤   ││
    // 88.00 ┤   │╰╮
    // 87.00 ┤   │ │
    // 86.00 ┤   ╭╮│
    // 85.00 ┤  ╭│││
    // 84.00 ┤  ││││
    // 83.00 ┤  ││╰╮
    // 82.00 ┤  ││ │╮
    // 81.00 ┤  ││ ││
    // 80.00 ┤╌╌╭╯╌││╌
    // 79.00 ┤  │  ││
    // 78.00 ┤ ╭│  ││
    // 77.00 ┤ ││  ╰╮
    // 76.00 ┤ ││   │
    // 75.00 ┤╌││╌╌╌│╌
    // 74.00 ┤ ╭╯   │
    // 73.00 ┤ │    │
    // 72.00 ┼╮│    │
    // 71.00 ┤││    ╰
    // 70.00 ┤││
    // 69.00 ┤││
    // 68.00 ┤╰╯
    // 67.00 ┤│
    // 66.00 ┤│
    // 65.00 ┼╯
}

fn threshold_multi_series() {
    // Series 0 oscillates between 65 and 91.
    // Series 1 oscillates between 68 and 86.
    // Both series share the same Y-axis space, making the graph
    // look like two overlapping curves rather than isolated shapes.
    let s1 = vec![65.0, 70.0, 78.0, 85.0, 91.0, 88.0, 82.0, 76.0];
    let s2 = vec![72.0, 68.0, 74.0, 80.0, 86.0, 83.0, 77.0, 71.0];

    let graph = plot_many(
        &[&s1, &s2],
        Config::default()
            .series_colors(&[AnsiColor::BLUE, AnsiColor::GREEN])
            // Threshold at 80.0 targeting series 0 (range 65–91).
            // Inherits BLUE from series_colors since no explicit color is set.
            .threshold(Threshold {
                series_index: 0,
                ..Threshold::new(80.0)
            })
            // Threshold at 75.0 targeting series 1 (range 68–86).
            // Inherits GREEN from series_colors since no explicit color is set.
            .threshold(Threshold {
                series_index: 1,
                ..Threshold::new(75.0)
            })
            // Threshold at 95.0 targeting series 1 (range 68–86).
            // Should NOT appear — 95.0 is above series 1's maximum of 86.0,
            // so the visibility rule skips it even though it would fit on
            // the global graph scale if series 0 reached that high.
            .threshold(Threshold {
                series_index: 1,
                ..Threshold::new(95.0)
            }),
    );

    println!("{}", graph);
}

