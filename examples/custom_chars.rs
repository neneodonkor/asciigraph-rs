use asciigraph::{plot_many, Config, AnsiColor};
fn main() {
    let mut data1: Vec<f64> = Vec::new();
    let mut data2: Vec<f64> = Vec::new();
    let mut data3: Vec<f64> = Vec::new();

    for i in 0..30 {
        data1.push((f64::from(i) * 0.2).sin() * 5.0 + 10.0);   // sin * amplitude + offset
        data2.push((f64::from(i) * 0.2).cos() * 3.0 + 10.0);   // cos, different amplitude
        data3.push((f64::from(i) * 0.3).sin() * 4.0 + 8.0);
    }

    //----------------------------------------------------------------------------------------------

    // Example 1: Using default box-drawing characters
    println!("Example 1: Default box-drawing characters");
    println!("=========================================");

    let graph1 = plot_many(
        &*vec![&data1[..]],
        Config::default()
            .height(10)
            .caption("Default Characters")
    );

    println!("{graph1}");
    println!();

    //----------------------------------------------------------------------------------------------

    // Example 2: Using asterisks for one series
    println!("Example 2: Custom asterisk characters");
    println!("=====================================");

    let graph2 = plot_many(
        &*vec![&data1[..]],
        Config::default()
            .series_chars(&[asciigraph::create_char_set('*')])
            .caption("Asterisk Characters")
    );

    println!("{graph2}");
    println!();

    //----------------------------------------------------------------------------------------------

    // Example 3: Using dots for one series
    println!("Example 3: Custom dot characters");
    println!("================================");

    let graph3 = plot_many(
        &*vec![&data1[..]],
        Config::default()
            .height(10)
            .series_chars(&[asciigraph::create_char_set('•')])
            .series_colors(&[AnsiColor::GREEN])
            .caption("Dot Characters (Green)")
    );

    println!("{graph3}");
    println!();

    //----------------------------------------------------------------------------------------------

    // Example 4: Multiple series with different characters
    println!("Example 4: Multiple series with different characters");
    println!("====================================================");

    let graph4 = plot_many(
        &*vec![&data1[..], &data2[..], &data3[..]],
        Config::default()
            .height(12)
            .series_chars(&[
                asciigraph::create_char_set('*'),
                asciigraph::create_char_set('#'),
                asciigraph::create_char_set('+'),
            ])
            .series_colors(&[AnsiColor::RED, AnsiColor::GREEN, AnsiColor::BLUE])
            .series_legends(&["Series 1 (*)", "Series 2 (#)", "Series 3 (+)"])
            .caption("Three Series with Different Characters")
    );

    println!("{graph4}");
    println!();

    //----------------------------------------------------------------------------------------------

    // Example 5: Partial character set (some fields use defaults)
    println!("Example 5: Partial character set (mixed with defaults)");
    println!("======================================================");

    let partial_set = asciigraph::CharSet{
        horizontal:   '=',
        vertical_line: '|',
        ..Default::default()
    };

    let graph5 = plot_many(
        &*vec![&data1[..]],
        Config::default()
            .height(10)
            .series_chars(&[partial_set])
            .series_colors(&[AnsiColor::CYAN])
            .caption("Partial CharSet (= and | with default corners)")
    );

    println!("{graph5}");
    println!();

    //----------------------------------------------------------------------------------------------

    // Example 6: Using simple ASCII characters
    println!("Example 6: Simple ASCII-only characters");
    println!("=======================================");

    let ascii_set = asciigraph::CharSet{
        horizontal:   '-',
        vertical_line: '|',
        arc_down_right: '/',
        arc_down_left:  '\\',
        arc_up_right:   '\\',
        arc_up_left:    '/',
        end_cap:       '-',
        start_cap:     '-',
        ..Default::default()
    };

    let graph6 = plot_many(
        &*vec![&data1[..]],
        Config::default()
            .height(10)
            .series_chars(&[ascii_set])
            .caption("ASCII-only Characters")
    );

    println!("{graph6}");
}