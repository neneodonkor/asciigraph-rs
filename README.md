# Rust Port of asciigraph

This repository was created to port the popular Golang project **[guptarohit/asciigraph](https://github.com/guptarohit/asciigraph)** into Rust.

The main goal of this project is to use it as a **learning opportunity** to improve my understanding of Rust, its ecosystem, ownership model, traits, error handling, and idiomatic patterns.

### Basic graph

```rust
use asciigraph::{plot, Config};

fn main() {
    let data = vec![3.0, 4.0, 9.0, 6.0, 2.0, 4.0, 5.0, 8.0, 5.0, 10.0, 2.0, 7.0, 2.0, 5.0, 6.0];
    let graph = plot(&data, Config::default());
    println!("{}", graph);
}
```

Running this example would render the following graph:
```bash
  10.00 ┤        ╭╮
   9.00 ┤ ╭╮     ││
   8.00 ┤ ││   ╭╮││
   7.00 ┤ ││   ││││╭╮
   6.00 ┤ │╰╮  ││││││ ╭
   5.00 ┤ │ │ ╭╯╰╯│││╭╯
   4.00 ┤╭╯ │╭╯   ││││
   3.00 ┼╯  ││    ││││
   2.00 ┤   ╰╯    ╰╯╰╯
```

### Multiple Series

```rust
use asciigraph::{plot_many, Config};

fn main() {
    let s1 = vec![0.0, 1.0, 2.0, 3.0, 3.0, 3.0, 2.0, 0.0];
    let s2 = vec![5.0, 4.0, 2.0, 1.0, 4.0, 6.0, 6.0];
    let data: Vec<&[f64]> = vec![&s1, &s2];

    let graph = plot_many(&data, Config::default());
    println!("{}", graph);
}
```

Running this example would render the following graph:
```bash
 6.00 ┤    ╭─
 5.00 ┼╮   │
 4.00 ┤╰╮ ╭╯
 3.00 ┤ │╭│─╮
 2.00 ┤ ╰╮│ ╰╮
 1.00 ┤╭╯╰╯  │
 0.00 ┼╯     ╰
```

### Custom Y-axis value formatting

Use `YAxisValueFormatter(...)` to control how values printed on the Y-axis are rendered.
This is useful for human-readable units like bytes, durations, or domain-specific labels.

```rust
use asciigraph::{plot, Config};

fn main() {
    let data = vec![
        30.0 * 1024.0 * 1024.0 * 1024.0,
        70.0 * 1024.0 * 1024.0 * 1024.0,
        2.0 * 1024.0 * 1024.0 * 1024.0,
    ];

    let graph = plot(
        &data,
        Config::default()
            .height(5)
            .width(45)
            .y_axis_value_formatter(Box::new(|v: f64| {
                format!("{:.2} GiB", v / 1024.0 / 1024.0 / 1024.0)
            })),
    );

    println!("{}", graph);
}
```

Running this example would render the following graph:
```bash
 70.00 GiB ┤                 ╭──────╮
 56.40 GiB ┤         ╭───────╯      ╰────╮
 42.80 GiB ┤  ╭──────╯                   ╰───╮
 29.20 GiB ┼──╯                              ╰────╮
 15.60 GiB ┤                                      ╰───╮
  2.00 GiB ┤                                          ╰─
```
