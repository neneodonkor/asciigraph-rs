use asciigraph::{plot, Config};

fn main() {
    let data: Vec<f64> = (1..=20).map(|x| x as f64).collect();

    // No x_axis_tick_count call — the library calculates it automatically.
    let graph = plot(
        &data,
        Config::default().x_axis_range(0.0, 100.0),
    );

    println!("{}", graph);
    // Output:
    // 20.00 ┤                  ╭
    // 19.00 ┤                 ╭╯
    // 18.00 ┤                ╭╯
    // 17.00 ┤               ╭╯
    // 16.00 ┤              ╭╯
    // 15.00 ┤             ╭╯
    // 14.00 ┤            ╭╯
    // 13.00 ┤           ╭╯
    // 12.00 ┤          ╭╯
    // 11.00 ┤         ╭╯
    // 10.00 ┤        ╭╯
    //  9.00 ┤       ╭╯
    //  8.00 ┤      ╭╯
    //  7.00 ┤     ╭╯
    //  6.00 ┤    ╭╯
    //  5.00 ┤   ╭╯
    //  4.00 ┤  ╭╯
    //  3.00 ┤ ╭╯
    //  2.00 ┤╭╯
    //  1.00 ┼╯
    //       └┬───┬───┬──┬───┬───┬
    //        0  20  40 60  80  100
}