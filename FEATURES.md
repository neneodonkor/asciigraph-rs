# Features

This document covers all configuration options and features available in
`asciigraph-rs`. For installation and basic usage, see the [README](README.md).

All features are configured through the `Config` builder — start with
`Config::default()` and chain the methods for the options you want.

### Zero-line Highlighting

Opt in to a horizontal reference line at Y = 0.0 by passing a `ZeroLine` value
to `Config::zero_line()`. The line is only rendered when the data range straddles
zero — if all values are positive or all negative, the option has no effect.

```rust
use asciigraph::{plot, Config, ZeroLine};

fn main() {
    let data = vec![
        3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -3.0, -2.0, -1.0, 0.0,
        1.0, 2.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -1.0, 0.0,
        1.0, 3.0, 5.0, 3.0, 1.0, -1.0, -3.0, -5.0, -3.0, -1.0,
    ];
    let graph = plot(&data, Config::default().zero_line(ZeroLine::new()));
    println!("{}", graph);
}
```

Running this example renders the following graph:

```
  5.00 ┤                     ╭╮
  4.00 ┤                     ││
  3.00 ┼╮          ╭╮       ╭╯╰╮
  2.00 ┤╰╮        ╭╯╰╮      │  │
  1.00 ┤ ╰╮      ╭╯  ╰╮    ╭╯  ╰╮
  0.00 ┤──╰╮────╭╯────╰╮──╭╯────│─────
 -1.00 ┤   ╰╮  ╭╯      ╰╮╭╯     ╰╮  ╭
 -2.00 ┤    ╰╮╭╯        ╰╯       │  │
 -3.00 ┤     ╰╯                  ╰╮╭╯
 -4.00 ┤                          ││
 -5.00 ┤                          ╰╯
```

The `─` characters along the `0.00` row are the zero line filling the empty
cells. Series arc characters take priority and render on top of the zero line
where they overlap.

To render the zero line in a specific color:

```rust
use asciigraph::{plot, Config, ZeroLine, AnsiColor};

fn main() {
    let graph = plot(
        &data,
        Config::default().zero_line(ZeroLine::with_color(AnsiColor::RED)),
    );
    println!("{}", graph);
}
````

### Threshold Lines

Add one or more horizontal reference lines at user-specified Y values using
`Config::threshold()`. Each threshold is rendered as a dashed line (`╌`) and
is associated with a specific data series via `series_index`, which defaults
to `0` (the first series). Call `.threshold()` multiple times to add more
than one line.

Two rules are applied before a threshold is drawn. The **visibility rule**
ensures the threshold value falls within the min/max range of its associated
series specifically — not just the global graph range. This means a threshold
that is meaningful for one series will not appear as a spurious line when
another series happens to cover that value range. The **color inheritance
rule** automatically applies the associated series' color to the threshold
line when no explicit color is set, creating a natural visual association
between a threshold and the line it annotates. An explicitly set color always
takes priority over the inherited series color.

#### Single series

```rust
use asciigraph::{plot, Config, Threshold, AnsiColor};

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
}
```

Running this example renders the following graph:

```
 94.00 ┤             ╭╮
 93.00 ┤             ││
 92.00 ┤             ││
 91.00 ┤    ╭╮       ││
 90.00 ┤╌╌╌╌││╌╌╌╌╌╌╌││╌╌╌╌╌
 89.00 ┤    ││      ╭╯│
 88.00 ┤    │╰╮     │ │
 87.00 ┤    │ │     │ ╰╮
 86.00 ┤    │ │     │  │
 85.00 ┤   ╭╯ │     │  │
 84.00 ┤   │  │     │  │
 83.00 ┤   │  │     │  │
 82.00 ┤   │  │    ╭╯  │
 81.00 ┤   │  │    │   │
 80.00 ┤╌╌╌│╌╌│╌╌╌╌│╌╌╌│╌╌╌╌
 79.00 ┤   │  │    │   ╰╮
 78.00 ┤  ╭╯  │    │    │
 77.00 ┤  │   │    │    │
 76.00 ┤  │   ╰╮   │    │
 75.00 ┤  │    │  ╭╯    │
 74.00 ┤  │    │  │     │
 73.00 ┤  │    │  │     │
 72.00 ┤ ╭╯    │  │     │
 71.00 ┤ │     │  │     ╰╮
 70.00 ┤ │     ╰╮ │      │
 69.00 ┤ │      │ │      │
 68.00 ┤ │      │╭╯      │
 67.00 ┤ │      ││       │
 66.00 ┤ │      ││       ╰╮
 65.00 ┤╭╯      ││        │
 64.00 ┤│       ╰╯        │
 63.00 ┤│                 │
 62.00 ┤│                 │
 61.00 ┤│                 │
 60.00 ┼╯                 ╰
```

#### Multi-series with visibility and color inheritance

In a multi-series graph, each threshold can be targeted at a specific series.
The threshold at 80.0 is associated with series 0 and inherits its BLUE color
automatically. The threshold at 75.0 is associated with series 1 and inherits
GREEN. The third threshold at 95.0 targeting series 1 is silently skipped
because 95.0 exceeds series 1's maximum value of 86.0.

```rust
use asciigraph::{plot_many, Config, Threshold, AnsiColor};

fn main() {
    let s1 = vec![65.0, 70.0, 78.0, 85.0, 91.0, 88.0, 82.0, 76.0];
    let s2 = vec![72.0, 68.0, 74.0, 80.0, 86.0, 83.0, 77.0, 71.0];

    let graph = plot_many(
        &[&s1, &s2],
        Config::default()
            .series_colors(&[AnsiColor::BLUE, AnsiColor::GREEN])
            // Inherits BLUE from series 0.
            .threshold(Threshold { series_index: 0, ..Threshold::new(80.0) })
            // Inherits GREEN from series 1.
            .threshold(Threshold { series_index: 1, ..Threshold::new(75.0) })
            // Skipped — 95.0 is outside series 1's range of 68–86.
            .threshold(Threshold { series_index: 1, ..Threshold::new(95.0) }),
    );

    println!("{}", graph);
}
```

Running this example renders the following graph:

```
 91.00 ┤   ╭╮
 90.00 ┤   ││
 89.00 ┤   ││
 88.00 ┤   │╰╮
 87.00 ┤   │ │
 86.00 ┤   ╭╮│
 85.00 ┤  ╭│││
 84.00 ┤  ││││
 83.00 ┤  ││╰╮
 82.00 ┤  ││ │╮
 81.00 ┤  ││ ││
 80.00 ┤╌╌╭╯╌││╌
 79.00 ┤  │  ││
 78.00 ┤ ╭│  ││
 77.00 ┤ ││  ╰╮
 76.00 ┤ ││   │
 75.00 ┤╌││╌╌╌│╌
 74.00 ┤ ╭╯   │
 73.00 ┤ │    │
 72.00 ┼╮│    │
 71.00 ┤││    ╰
 70.00 ┤││
 69.00 ┤││
 68.00 ┤╰╯
 67.00 ┤│
 66.00 ┤│
 65.00 ┼╯
```

Series arc characters always render on top of threshold lines where they
overlap. A threshold whose value falls outside its associated series' range
is silently skipped regardless of whether it falls within the global graph
range.


### Moving Average Overlay

Add a smoothed trend line on top of your data using `Config::moving_average()`.
The moving average is computed over a sliding window and rendered as an
additional series. Pair it with `series_colors` to visually distinguish the
smoothed series from the raw data.

```rust
use asciigraph::{plot, Config, AnsiColor};

fn main() {
    let data = vec![
        3.0, 1.0, 5.0, 2.0, 8.0, 4.0, 7.0, 2.0, 6.0, 3.0,
        9.0, 4.0, 6.0, 2.0, 7.0, 3.0, 8.0, 1.0, 5.0, 3.0,
    ];

    let graph = plot(
        &data,
        Config::default()
            .moving_average(5)
            .series_colors(&[AnsiColor::DEFAULT, AnsiColor::YELLOW]),
    );

    println!("{}", graph);
}
```

Running this example renders the following graph:

```
 9.00 ┤         ╭╮
 8.00 ┤   ╭╮    ││    ╭╮
 7.00 ┤   ││╭╮  ││  ╭╮││
 6.00 ┤   ││││╭╮╭╮╭╮││││
 5.00 ┤ ╭╮╭──╮╭─╯╰╯│╭╮╭╮╭╮
 4.00 ┤ ╭─╯╰╯╰╯││╰╯╰╯╰╯╰─╮
 3.00 ┼─╯││  ││╰╯  ││╰╯││╰
 2.00 ┤││╰╯  ╰╯    ╰╯  ││
 1.00 ┤╰╯              ╰╯
```

The raw data is rendered in the default color. The yellow series is the
5-point moving average, showing the underlying trend with short-term noise
smoothed out.

### Auto Tick Count

When using the X-axis, you do not need to specify the number of ticks.
If `x_axis_tick_count` is not set, the library automatically calculates
a sensible number based on the available graph width and the estimated
label width.

```rust
use asciigraph::{plot, Config};

fn main() {
    let data: Vec<f64> = (1..=20).map(|x| x as f64).collect();

    let graph = plot(
        &data,
        Config::default().x_axis_range(0.0, 100.0),
    );

    println!("{}", graph);
}
```

Running this example renders the following graph:

```
 20.00 ┤                  ╭
 19.00 ┤                 ╭╯
 18.00 ┤                ╭╯
 17.00 ┤               ╭╯
 16.00 ┤              ╭╯
 15.00 ┤             ╭╯
 14.00 ┤            ╭╯
 13.00 ┤           ╭╯
 12.00 ┤          ╭╯
 11.00 ┤         ╭╯
 10.00 ┤        ╭╯
  9.00 ┤       ╭╯
  8.00 ┤      ╭╯
  7.00 ┤     ╭╯
  6.00 ┤    ╭╯
  5.00 ┤   ╭╯
  4.00 ┤  ╭╯
  3.00 ┤ ╭╯
  2.00 ┤╭╯
  1.00 ┼╯
       └┬───┬───┬──┬───┬───┬
        0  20  40 60  80  100
```

You can still override the automatic calculation by calling
`x_axis_tick_count()` explicitly when you need precise control.

### X and Y Axis Labels

Add descriptive labels to your axes using `Config::y_axis_label()` and
`Config::x_axis_label()`. The Y-axis label renders flush left above the
graph body. The X-axis label renders inline on the same row as the axis
line, to the right of the tick marks. Note that `x_axis_label` only
appears when `x_axis_range` is also configured.

```rust
use asciigraph::{plot, Config};

fn main() {
    let data = vec![
        3.0, 1.0, 5.0, 2.0, 8.0, 4.0, 7.0, 2.0, 6.0, 3.0,
        9.0, 4.0, 6.0, 2.0, 7.0, 3.0, 8.0, 1.0, 5.0, 3.0,
    ];

    let graph = plot(
        &data,
        Config::default()
            .x_axis_range(0.0, 100.0)
            .y_axis_label("Memory (MB)")
            .x_axis_label("Time (seconds)"),
    );

    println!("{}", graph);
}
```
Running this example renders the following graph:

```
 Memory (MB)
 9.00 ┤         ╭╮
 8.00 ┤   ╭╮    ││    ╭╮
 7.00 ┤   ││╭╮  ││  ╭╮││
 6.00 ┤   ││││╭╮││╭╮││││
 5.00 ┤ ╭╮││││││││││││││╭╮
 4.00 ┤ │││╰╯││││╰╯│││││││
 3.00 ┼╮│││  ││╰╯  ││╰╯││╰
 2.00 ┤││╰╯  ╰╯    ╰╯  ││
 1.00 ┤╰╯              ╰╯
      └┬───┬───┬──┬───┬───┬
      0  20  40 60  80  100   Time (seconds)
```

### Serde Support

Enable the `serde` feature flag to serialize and deserialize `Config` and
related types to and from any Serde-compatible format such as JSON or TOML.

```toml
# Cargo.toml
asciigraph = { version = "0.1.5", features = ["serde"] }
```

```rust
use asciigraph::Config;

fn main() {
    let config = Config::default().height(10).caption("My graph");
    let json = serde_json::to_string_pretty(&config).unwrap();
    let restored: Config = serde_json::from_str(&json).unwrap();
}
```

Note that formatter closure fields (`x_axis_value_formatter` and
`y_axis_value_formatter`) are skipped during serialization and restored
as `None` on deserialization. All other fields roundtrip faithfully.

### Statistical Annotations

Add automatically computed reference lines to your graph using
`Config::stat_annotations()`. The library computes the minimum, maximum,
mean, median, and standard deviation directly from your data — no manual
calculation required. Each annotation is rendered as a horizontal line
using a distinct dashed character, with an inline label showing the
statistic name and its computed value.

Each annotation type uses a different line character so they are visually
distinguishable at a glance: minimum and maximum use `╌`, mean uses `┄`,
median uses `╍`, and standard deviation renders as two dotted lines (`·`)
at one standard deviation above and below the mean, labeled `+σ` and `-σ`.
When two annotation values are close enough to map to the same grid row,
their labels are stacked on that row and separated by a comma.

In a multi-series graph, set `series_index` on the `StatAnnotations` struct
to control which series the statistics are computed from. The default is `0`,
which targets the first series.

```rust
use asciigraph::{plot, Config, StatAnnotations, AnsiColor};

fn main() {
    let data = vec![
        3.0, 1.0, 5.0, 2.0, 8.0, 4.0, 7.0, 2.0, 6.0, 3.0,
        9.0, 4.0, 6.0, 2.0, 7.0, 3.0, 8.0, 1.0, 5.0, 3.0,
    ];

    let graph = plot(
        &data,
        Config::default()
            .stat_annotations(StatAnnotations::with_color(AnsiColor::YELLOW)),
    );

    println!("{}", graph);
}
```

Running this example renders the following graph:

```
 9.00 ┤╌╌╌╌╌╌╌╌╌╭╮╌╌╌╌╌╌╌╌  max 9.00
 8.00 ┤   ╭╮    ││    ╭╮
 7.00 ┤···││╭╮··││··╭╮││··  +σ 6.85
 6.00 ┤   ││││╭╮││╭╮││││
 5.00 ┤ ╭╮││││││││││││││╭╮
 4.00 ┤┄│││╰╯││││╰╯│││││││  mean 4.45, med 4.00
 3.00 ┼╮│││  ││╰╯  ││╰╯││╰
 2.00 ┤││╰╯··╰╯····╰╯··││·  -σ 2.05
 1.00 ┤╰╯╌╌╌╌╌╌╌╌╌╌╌╌╌╌╰╯╌  min 1.00
```

To annotate only specific statistics, use struct literal syntax to set
individual flags:

```rust
use asciigraph::{StatAnnotations, AnsiColor};

fn main() {
    // Show only min and max in red.
    let annotations = StatAnnotations {
        show_min:     true,
        show_max:     true,
        show_mean:    false,
        show_median:  false,
        show_std_dev: false,
        series_index: 0,
        color:        AnsiColor::RED,
    };

    // Annotate the second series in a multi-series graph.
    let annotations = StatAnnotations {
        series_index: 1,
        ..StatAnnotations::new()
    };
}
```