# Features

This document covers all configuration options and features available in
`asciigraph-rs`. For installation and basic usage, see the [README](README.md).

All features are configured through the `Config` builder — start with
`Config::default()` and chain the methods for the options you want.

### Zero-line highlighting

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

### Threshold lines

Add one or more horizontal reference lines at user-specified Y values using
`Config::threshold()`. Each threshold is rendered as a dashed line (`╌`) and
carries its own color independently. Call `.threshold()` multiple times to add
more than one line.

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

Thresholds outside the visible Y range are silently ignored. Series arc
characters always render on top of threshold lines where they overlap.

### Moving average overlay

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

### Auto tick count

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