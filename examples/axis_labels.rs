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

    // Output:
    // Memory (MB)
    // 9.00 ┤         ╭╮
    // 8.00 ┤   ╭╮    ││    ╭╮
    // 7.00 ┤   ││╭╮  ││  ╭╮││
    // 6.00 ┤   ││││╭╮││╭╮││││
    // 5.00 ┤ ╭╮││││││││││││││╭╮
    // 4.00 ┤ │││╰╯││││╰╯│││││││
    // 3.00 ┼╮│││  ││╰╯  ││╰╯││╰
    // 2.00 ┤││╰╯  ╰╯    ╰╯  ││
    // 1.00 ┤╰╯              ╰╯
    //      └┬───┬───┬──┬───┬───┬
    //      0  20  40 60  80  100   Time (seconds)
}