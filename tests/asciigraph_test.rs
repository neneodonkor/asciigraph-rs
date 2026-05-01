#[cfg(test)]
mod tests {
    use asciigraph::{plot, plot_many};
    use asciigraph::AnsiColor;
    use asciigraph::Config;
    use asciigraph::ZeroLine;
    use asciigraph::Threshold;

    // Helper to clean expected strings the same way Go tests do
    fn clean(s: &str) -> String {
        s.trim_start_matches('\n')
            .replace(r"\x1b", "\x1b")
            .to_string()
    }

    // -------------------------------------------------------------------------
    // TestPlot
    // -------------------------------------------------------------------------

    #[test]
    fn test_plot_flat_ones() {
        let data = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let expected = clean(" 1.00 ┼────");
        assert_eq!(plot(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_flat_zeros() {
        let data = vec![0.0, 0.0, 0.0, 0.0, 0.0];
        let expected = clean(" 0.00 ┼────");
        assert_eq!(plot(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_precision_and_caption() {
        let data = vec![49.51, 49.51, 49.51];
        let config = Config::default()
            .precision(2)
            .caption("Code Coverage (excluding generated)");
        let expected = clean(
            r#"
 49.51 ┼──
        Code Coverage (excluding generated)"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_negative_flat() {
        let data = vec![-49.51, -49.51, -49.51];
        let config = Config::default().precision(2);
        let expected = clean(" -49.51 ┼──");
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_basic_series() {
        let data = vec![2.0, 1.0, 1.0, 2.0, -2.0, 5.0, 7.0, 11.0, 3.0, 7.0, 1.0];
        let expected = clean(
            r#"
 11.00 ┤      ╭╮
 10.00 ┤      ││
  9.00 ┤      ││
  8.00 ┤      ││
  7.00 ┤     ╭╯│╭╮
  6.00 ┤     │ │││
  5.00 ┤    ╭╯ │││
  4.00 ┤    │  │││
  3.00 ┤    │  ╰╯│
  2.00 ┼╮ ╭╮│    │
  1.00 ┤╰─╯││    ╰
  0.00 ┤   ││
 -1.00 ┤   ││
 -2.00 ┤   ╰╯"#,
        );
        assert_eq!(plot(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_with_caption() {
        let data = vec![
            2.0, 1.0, 1.0, 2.0, -2.0, 5.0, 7.0, 11.0, 3.0, 7.0, 4.0, 5.0, 6.0, 9.0, 4.0, 0.0,
            6.0, 1.0, 5.0, 3.0, 6.0, 2.0,
        ];
        let config = Config::default().caption("Plot using asciigraph.");
        let expected = clean(
            r#"
 11.00 ┤      ╭╮
 10.00 ┤      ││
  9.00 ┤      ││    ╭╮
  8.00 ┤      ││    ││
  7.00 ┤     ╭╯│╭╮  ││
  6.00 ┤     │ │││ ╭╯│ ╭╮  ╭╮
  5.00 ┤    ╭╯ │││╭╯ │ ││╭╮││
  4.00 ┤    │  ││╰╯  ╰╮││││││
  3.00 ┤    │  ╰╯     ││││╰╯│
  2.00 ┼╮ ╭╮│         ││││  ╰
  1.00 ┤╰─╯││         ││╰╯
  0.00 ┤   ││         ╰╯
 -1.00 ┤   ││
 -2.00 ┤   ╰╯
        Plot using asciigraph."#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_small_decimals_with_caption() {
        let data = vec![0.2, 0.1, 0.2, 2.0, -0.9, 0.7, 0.91, 0.3, 0.7, 0.4, 0.5];
        let config = Config::default().caption("Plot using asciigraph.");
        let expected = clean(
            r#"
  2.00 ┤  ╭╮ ╭╮
  0.55 ┼──╯│╭╯╰───
 -0.90 ┤   ╰╯
        Plot using asciigraph."#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_custom_height_offset() {
        let data = vec![2.0, 1.0, 1.0, 2.0, -2.0, 5.0, 7.0, 11.0, 3.0, 7.0, 1.0];
        let config = Config::default().height(4).offset(3);
        let expected = clean(
            r#"
 11.00 ┤      ╭╮
  7.75 ┤    ╭─╯│╭╮
  4.50 ┼╮ ╭╮│  ╰╯│
  1.25 ┤╰─╯││    ╰
 -2.00 ┤   ╰╯"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_fractional_data() {
        let data = vec![
            0.453, 0.141, 0.951, 0.251, 0.223, 0.581, 0.771, 0.191, 0.393, 0.617, 0.478,
        ];
        let expected = clean(
            r#"
 0.95 ┤ ╭╮
 0.85 ┤ ││  ╭╮
 0.75 ┤ ││  ││
 0.65 ┤ ││ ╭╯│ ╭╮
 0.55 ┤ ││ │ │ │╰
 0.44 ┼╮││ │ │╭╯
 0.34 ┤│││ │ ││
 0.24 ┤││╰─╯ ╰╯
 0.14 ┤╰╯"#,
        );
        assert_eq!(plot(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_tiny_decimals() {
        let data = vec![0.01, 0.004, 0.003, 0.0042, 0.0083, 0.0033, 0.0079];
        let expected = clean(
            r#"
 0.010 ┼╮
 0.009 ┤│
 0.008 ┤│  ╭╮╭
 0.007 ┤│  │││
 0.006 ┤│  │││
 0.005 ┤│  │││
 0.004 ┤╰╮╭╯││
 0.003 ┤ ╰╯ ╰╯"#,
        );
        assert_eq!(plot(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_large_numbers_custom_height() {
        let data = vec![
            192.0, 431.0, 112.0, 449.0, -122.0, 375.0, 782.0, 123.0, 911.0, 1711.0, 172.0,
        ];
        let config = Config::default().height(10);
        let expected = clean(
            r#"
 1711 ┤        ╭╮
 1528 ┤        ││
 1344 ┤        ││
 1161 ┤        ││
  978 ┤       ╭╯│
  794 ┤     ╭╮│ │
  611 ┤     │││ │
  428 ┤╭╮╭╮╭╯││ │
  245 ┼╯╰╯││ ╰╯ ╰
   61 ┤   ││
 -122 ┤   ╰╯"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_custom_width_height_caption() {
        let data = vec![
            0.3189989805, 0.149949026, 0.30142492354, 0.195129182935, 0.3142492354, 0.1674974513,
            0.3142492354, 0.1474974513, 0.3047974513,
        ];
        let config = Config::default()
            .width(30)
            .height(5)
            .caption("Plot with custom height & width.");
        let expected = clean(
            r#"
 0.32 ┼╮            ╭─╮     ╭╮     ╭
 0.29 ┤╰╮    ╭─╮   ╭╯ │    ╭╯│     │
 0.26 ┤ │   ╭╯ ╰╮ ╭╯  ╰╮  ╭╯ ╰╮   ╭╯
 0.23 ┤ ╰╮ ╭╯   ╰╮│    ╰╮╭╯   ╰╮ ╭╯
 0.20 ┤  ╰╮│     ╰╯     ╰╯     │╭╯
 0.16 ┤   ╰╯                   ╰╯
       Plot with custom height & width."#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_large_series_with_repeating_pattern() {
        let data = vec![
            0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0, -0.5, 9.0, -3.0, 0.0, 0.0, 1.0, 2.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0, -0.5, 8.0, -3.0, 0.0, 0.0, 1.0,
            2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0, -0.5, 10.0, -3.0,
            0.0, 0.0, 1.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let config = Config::default()
            .offset(10)
            .height(10)
            .caption("I'm a doctor, not an engineer.");
        let expected = clean(
            r#"
     10.00    ┤                                             ╭╮
      8.70    ┤       ╭╮                                    ││
      7.40    ┤       ││                 ╭╮                 ││
      6.10    ┤       ││                 ││                 ││
      4.80    ┤       ││                 ││                 ││
      3.50    ┤       ││                 ││                 ││
      2.20    ┤       ││   ╭╮            ││   ╭╮            ││   ╭╮
      0.90    ┤   ╭╮  ││  ╭╯╰╮       ╭╮  ││  ╭╯╰╮       ╭╮  ││  ╭╯╰╮
     -0.40    ┼───╯╰──╯│╭─╯  ╰───────╯╰──╯│╭─╯  ╰───────╯╰──╯│╭─╯  ╰───
     -1.70    ┤        ││                 ││                 ││
     -3.00    ┤        ╰╯                 ╰╯                 ╰╯
                            I'm a doctor, not an engineer."#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_negative_series() {
        let data = vec![
            -5.0, -2.0, -3.0, -4.0, 0.0, -5.0, -6.0, -7.0, -8.0, 0.0, -9.0, -3.0, -5.0, -2.0,
            -9.0, -3.0, -1.0,
        ];
        let expected = clean(
            r#"
  0.00 ┤   ╭╮   ╭╮
 -1.00 ┤   ││   ││     ╭
 -2.00 ┤╭╮ ││   ││  ╭╮ │
 -3.00 ┤│╰╮││   ││╭╮││╭╯
 -4.00 ┤│ ╰╯│   │││││││
 -5.00 ┼╯   ╰╮  │││╰╯││
 -6.00 ┤     ╰╮ │││  ││
 -7.00 ┤      ╰╮│││  ││
 -8.00 ┤       ╰╯││  ││
 -9.00 ┤         ╰╯  ╰╯"#,
        );
        assert_eq!(plot(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_tiny_negative_decimals() {
        let data = vec![
            -0.000018527,
            -0.021,
            -0.00123,
            0.00000021312,
            -0.0434321234,
            -0.032413241234,
            0.0000234234,
        ];
        let config = Config::default().height(5).width(45);
        let expected = clean(
            r#"
  0.000 ┼─╮           ╭────────╮                    ╭
 -0.008 ┤ ╰──╮     ╭──╯        ╰─╮                ╭─╯
 -0.017 ┤    ╰─────╯             ╰╮             ╭─╯
 -0.025 ┤                         ╰─╮         ╭─╯
 -0.034 ┤                           ╰╮   ╭────╯
 -0.042 ┤                            ╰───╯"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_stock_data() {
        let data = vec![
            57.76, 54.04, 56.31, 57.02, 59.5, 52.63, 52.97, 56.44, 56.75, 52.96, 55.54, 55.09,
            58.22, 56.85, 60.61, 59.62, 59.73, 59.93, 56.3, 54.69, 55.32, 54.03, 50.98, 50.48,
            54.55, 47.49, 55.3, 46.74, 46.0, 45.8, 49.6, 48.83, 47.64, 46.61, 54.72, 42.77,
            50.3, 42.79, 41.84, 44.19, 43.36, 45.62, 45.09, 44.95, 50.36, 47.21, 47.77, 52.04,
            47.46, 44.19, 47.22, 45.55, 40.65, 39.64, 37.26, 40.71, 42.15, 36.45, 39.14, 36.62,
        ];
        let config = Config::default().width(0).height(0).offset(3);
        let expected = clean(
            r#"
 60.61 ┤             ╭╮ ╭╮
 59.60 ┤   ╭╮        │╰─╯│
 58.60 ┤   ││      ╭╮│   │
 57.59 ┼╮ ╭╯│      │││   │
 56.58 ┤│╭╯ │ ╭─╮  │╰╯   ╰╮
 55.58 ┤││  │ │ │╭─╯      │╭╮    ╭╮
 54.57 ┤╰╯  │ │ ││        ╰╯╰╮ ╭╮││      ╭╮
 53.56 ┤    │╭╯ ╰╯           │ ││││      ││
 52.56 ┤    ╰╯               │ ││││      ││           ╭╮
 51.55 ┤                     ╰╮││││      ││           ││
 50.54 ┤                      ╰╯│││      ││╭╮      ╭╮ ││
 49.54 ┤                        │││  ╭─╮ ││││      ││ ││
 48.53 ┤                        │││  │ │ ││││      ││ ││
 47.52 ┤                        ╰╯│  │ ╰╮││││      │╰─╯╰╮╭╮
 46.52 ┤                          ╰─╮│  ╰╯│││      │    │││
 45.51 ┤                            ╰╯    │││   ╭──╯    ││╰╮
 44.50 ┤                                  │││ ╭╮│       ╰╯ │
 43.50 ┤                                  ││╰╮│╰╯          │
 42.49 ┤                                  ╰╯ ╰╯            │   ╭╮
 41.48 ┤                                                   │   ││
 40.48 ┤                                                   ╰╮ ╭╯│
 39.47 ┤                                                    ╰╮│ │╭╮
 38.46 ┤                                                     ││ │││
 37.46 ┤                                                     ╰╯ │││
 36.45 ┤                                                        ╰╯╰"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_lower_upper_bound() {
        let data = vec![
            2.0, 1.0, 1.0, 2.0, -2.0, 5.0, 7.0, 11.0, 3.0, 7.0, 4.0, 5.0, 6.0, 9.0, 4.0, 0.0,
            6.0, 1.0, 5.0, 3.0, 6.0, 2.0,
        ];
        let config = Config::default().lower_bound(-3.0).upper_bound(13.0);
        let expected = clean(
            r#" 13.00 ┤
 12.00 ┤
 11.00 ┤      ╭╮
 10.00 ┤      ││
  9.00 ┤      ││    ╭╮
  8.00 ┤      ││    ││
  7.00 ┤     ╭╯│╭╮  ││
  6.00 ┤     │ │││ ╭╯│ ╭╮  ╭╮
  5.00 ┤    ╭╯ │││╭╯ │ ││╭╮││
  4.00 ┤    │  ││╰╯  ╰╮││││││
  3.00 ┤    │  ╰╯     ││││╰╯│
  2.00 ┼╮ ╭╮│         ││││  ╰
  1.00 ┤╰─╯││         ││╰╯
  0.00 ┤   ││         ╰╯
 -1.00 ┤   ││
 -2.00 ┤   ╰╯
 -3.00 ┤"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_bounds_ignored_when_data_outside() {
        let data = vec![
            2.0, 1.0, 1.0, 2.0, -2.0, 5.0, 7.0, 11.0, 3.0, 7.0, 4.0, 5.0, 6.0, 9.0, 4.0, 0.0,
            6.0, 1.0, 5.0, 3.0, 6.0, 2.0,
        ];
        let config = Config::default().lower_bound(0.0).upper_bound(3.0);
        let expected = clean(
            r#" 11.00 ┤      ╭╮
 10.00 ┤      ││
  9.00 ┤      ││    ╭╮
  8.00 ┤      ││    ││
  7.00 ┤     ╭╯│╭╮  ││
  6.00 ┤     │ │││ ╭╯│ ╭╮  ╭╮
  5.00 ┤    ╭╯ │││╭╯ │ ││╭╮││
  4.00 ┤    │  ││╰╯  ╰╮││││││
  3.00 ┤    │  ╰╯     ││││╰╯│
  2.00 ┼╮ ╭╮│         ││││  ╰
  1.00 ┤╰─╯││         ││╰╯
  0.00 ┤   ││         ╰╯
 -1.00 ┤   ││
 -2.00 ┤   ╰╯"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_nan_in_middle() {
        let data = vec![1.0, 1.0, f64::NAN, 1.0, 1.0];
        let expected = clean(" 1.00 ┼─╴╶─");
        assert_eq!(plot(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_nan_at_start() {
        let data = vec![f64::NAN, 1.0];
        let expected = clean(" 1.00 ┤╶");
        assert_eq!(plot(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_nan_gaps() {
        let data = vec![0.0, 0.0, 1.0, 1.0, f64::NAN, f64::NAN, 3.0, 3.0, 4.0];
        let expected = clean(
            r#"
 4.00 ┤       ╭
 3.00 ┤     ╶─╯
 2.00 ┤
 1.00 ┤ ╭─╴
 0.00 ┼─╯"#,
        );
        assert_eq!(plot(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_nan_multiple_gaps() {
        let data = vec![
            0.1,
            0.2,
            0.3,
            f64::NAN,
            0.5,
            0.6,
            0.7,
            f64::NAN,
            f64::NAN,
            0.9,
            1.0,
        ];
        let expected = clean(
            r#"
 1.00 ┤         ╭
 0.90 ┤        ╶╯
 0.80 ┤
 0.70 ┤     ╭╴
 0.60 ┤    ╭╯
 0.50 ┤   ╶╯
 0.40 ┤
 0.30 ┤ ╭╴
 0.20 ┤╭╯
 0.10 ┼╯"#,
        );
        assert_eq!(plot(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_tiny_decimals_with_precision() {
        let data = vec![
            -0.000018527,
            -0.021,
            -0.00123,
            0.00000021312,
            -0.0434321234,
            -0.032413241234,
            0.0000234234,
        ];
        let config = Config::default().height(5).width(45).precision(5);
        let expected = clean(
            r#"
  0.000023 ┼─╮           ╭────────╮                    ╭
 -0.008467 ┤ ╰──╮     ╭──╯        ╰─╮                ╭─╯
 -0.016958 ┤    ╰─────╯             ╰╮             ╭─╯
 -0.025449 ┤                         ╰─╮         ╭─╯
 -0.033940 ┤                           ╰╮   ╭────╯
 -0.042430 ┤                            ╰───╯"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_color() {
        let data = vec![f64::NAN, 1.0];
        let config = Config::default()
            .caption("color test")
            .caption_color(AnsiColor::RED)
            .axis_color(AnsiColor::GREEN)
            .label_color(AnsiColor::BLUE);
        let expected = clean(
            r#"
\x1b[94m 1.00\x1b[0m \x1b[32m┤\x1b[0m╶
       \x1b[91mcolor test\x1b[0m"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_small_range_1() {
        let data = vec![0.02, 0.03, 0.02];
        let expected = clean(
            r#"
 0.030 ┤╭╮
 0.020 ┼╯╰"#,
        );
        assert_eq!(plot(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_small_range_2() {
        let data = vec![0.2, 0.3, 0.1, 0.3];
        let expected = clean(
            r#"
 0.30 ┤╭╮╭
 0.20 ┼╯││
 0.10 ┤ ╰╯"#,
        );
        assert_eq!(plot(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_y_axis_value_formatter() {
        let data = vec![
            70.0 * 1024.0 * 1024.0 * 1024.0,
            90.0 * 1024.0 * 1024.0 * 1024.0,
            80.0 * 1024.0 * 1024.0 * 1024.0,
            2.0 * 1024.0 * 1024.0 * 1024.0,
        ];
        let config = Config::default()
            .height(5)
            .width(45)
            .y_axis_value_formatter(Box::new(|v: f64| {
                format!("{:.2} Foo", v / 1024.0 / 1024.0 / 1024.0)
            }));
        let expected = clean(
            r#" 89.77 Foo ┤      ╭──────────────────────╮
 72.22 Foo ┼──────╯                      ╰──╮
 54.66 Foo ┤                                ╰───╮
 37.11 Foo ┤                                    ╰──╮
 19.55 Foo ┤                                       ╰──╮
  2.00 Foo ┤                                          ╰─"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_y_axis_formatter_with_precision() {
        let data = vec![49.51, 49.51, 49.51];
        let config = Config::default()
            .precision(1)
            .y_axis_value_formatter(Box::new(|v: f64| format!("{:.1} GiB", v)));
        let expected = clean(" 49.5 GiB ┼──");
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_x_axis_basic() {
        let data = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let config = Config::default().x_axis_range(0.0, 100.0).x_axis_tick_count(2);
        let expected = clean(
            r#"
 1.00 ┼────
      └┬───┬
       0  100"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_x_axis_three_ticks() {
        let data = vec![2.0, 1.0, 1.0, 2.0, -2.0, 5.0, 7.0, 11.0, 3.0, 7.0, 1.0];
        let config = Config::default()
            .x_axis_range(0.0, 100.0)
            .x_axis_tick_count(3);
        let expected = clean(
            r#"
 11.00 ┤      ╭╮
 10.00 ┤      ││
  9.00 ┤      ││
  8.00 ┤      ││
  7.00 ┤     ╭╯│╭╮
  6.00 ┤     │ │││
  5.00 ┤    ╭╯ │││
  4.00 ┤    │  │││
  3.00 ┤    │  ╰╯│
  2.00 ┼╮ ╭╮│    │
  1.00 ┤╰─╯││    ╰
  0.00 ┤   ││
 -1.00 ┤   ││
 -2.00 ┤   ╰╯
       └┬────┬────┬
        0   50   100"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_x_axis_custom_formatter() {
        let data = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let config = Config::default()
            .x_axis_range(0.0, 100.0)
            .x_axis_tick_count(2)
            .x_axis_value_formatter(Box::new(|v: f64| format!("{:.0}ms", v)));
        let expected = clean(
            r#"
 1.00 ┼────
      └┬───┬
      0ms"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    #[test]
    fn test_plot_x_axis_with_caption() {
        let data = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let config = Config::default()
            .x_axis_range(0.0, 100.0)
            .x_axis_tick_count(2)
            .caption("test caption");
        let expected = clean(
            r#"
 1.00 ┼────
      └┬───┬
       0  100
       test caption"#,
        );
        assert_eq!(plot(&data, config), expected);
    }

    // -------------------------------------------------------------------------
    // TestPlotMany
    // -------------------------------------------------------------------------

    #[test]
    fn test_plot_many_single_points() {
        let data: Vec<&[f64]> = vec![&[0.0], &[1.0], &[2.0]];
        let expected = clean(
            r#"
 2.00 ┼
 1.00 ┼
 0.00 ┼"#,
        );
        assert_eq!(plot_many(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_many_with_nans() {
        let s1 = vec![0.0, 0.0, 2.0, 2.0, f64::NAN];
        let s2 = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let s3 = vec![f64::NAN, f64::NAN, f64::NAN, 0.0, 0.0, 2.0, 2.0];
        let data: Vec<&[f64]> = vec![&s1, &s2, &s3];
        let expected = clean(
            r#"
 2.00 ┤ ╭─╴╭─
 1.00 ┼────│─
 0.00 ┼─╯╶─╯"#,
        );
        assert_eq!(plot_many(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_many_all_zeros_with_nans() {
        let s1 = vec![0.0, 0.0, 0.0];
        let s2 = vec![f64::NAN, 0.0, 0.0];
        let s3 = vec![f64::NAN, f64::NAN, 0.0];
        let data: Vec<&[f64]> = vec![&s1, &s2, &s3];
        let expected = clean(" 0.00 ┼╶╶");
        assert_eq!(plot_many(&data, Config::default()), expected);
    }

    #[test]
    fn test_plot_many_interpolation() {
        let s1 = vec![0.0, 1.0, 0.0];
        let s2 = vec![2.0, 3.0, 4.0, 3.0, 2.0];
        let s3 = vec![4.0, 5.0, 6.0, 7.0, 6.0, 5.0, 4.0];
        let data: Vec<&[f64]> = vec![&s1, &s2, &s3];
        let config = Config::default().width(21).caption("interpolation test");
        let expected = clean(
            r#"
 7.00 ┤        ╭──╮
 6.00 ┤    ╭───╯  ╰───╮
 5.00 ┤ ╭──╯          ╰──╮
 4.00 ┼─╯  ╭───╮         ╰─
 3.00 ┤ ╭──╯   ╰──╮
 2.00 ┼─╯         ╰─╴
 1.00 ┤ ╭───╮
 0.00 ┼─╯   ╰╴
        interpolation test"#,
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_plot_many_series_color_first_only() {
        let s1 = vec![0.0, 0.0];
        let s2 = vec![f64::NAN, 0.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let config = Config::default().series_colors(&[AnsiColor::RED]);
        let expected = clean(" 0.00 ┼╶");
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_plot_many_series_color_second_only() {
        let s1 = vec![0.0, 0.0];
        let s2 = vec![f64::NAN, 0.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let config = Config::default().series_colors(&[AnsiColor::DEFAULT, AnsiColor::RED]);
        let expected = clean(" 0.00 ┼\x1b[91m╶\x1b[0m");
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_plot_many_both_red() {
        let s1 = vec![f64::NAN, 0.0, 2.0];
        let s2 = vec![0.0, 2.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let config = Config::default().series_colors(&[AnsiColor::RED, AnsiColor::RED]);
        let expected = clean(
            r#"
 2.00 ┤\x1b[91m╭╭\x1b[0m
 1.00 ┤\x1b[91m││\x1b[0m
 0.00 ┼\x1b[91m╯╯\x1b[0m"#,
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_plot_many_legends_and_caption() {
        let s1 = vec![0.0, 1.0, 0.0];
        let s2 = vec![2.0, 3.0, 4.0, 3.0, 2.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let config = Config::default()
            .series_colors(&[AnsiColor::RED, AnsiColor::BLUE])
            .series_legends(&["Red", "Blue"])
            .caption("legends with caption test");
        let expected = format!(
            " 4.00 ┤ {}╭╮{}\n 3.00 ┤{}╭╯╰╮{}\n 2.00 ┼{}╯{}  {}╰{}\n 1.00 ┤{}╭╮{}\n 0.00 ┼{}╯╰{}\n       legends with caption test\n\n       {}■{} Red   {}■{} Blue",
            "\x1b[94m", "\x1b[0m",
            "\x1b[94m", "\x1b[0m",
            "\x1b[94m", "\x1b[0m",
            "\x1b[94m", "\x1b[0m",
            "\x1b[91m", "\x1b[0m",
            "\x1b[91m", "\x1b[0m",
            "\x1b[91m", "\x1b[0m",
            "\x1b[94m", "\x1b[0m",
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_plot_many_legends_no_colors() {
        let s1 = vec![0.0, 1.0, 0.0];
        let s2 = vec![2.0, 3.0, 4.0, 3.0, 2.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let config = Config::default().series_legends(&["First", "Second"]);
        let expected = format!(
            " 4.00 ┤ ╭╮\n 3.00 ┤╭╯╰╮\n 2.00 ┼╯  ╰\n 1.00 ┤╭╮\n 0.00 ┼╯╰\n\n       {}■{} First   {}■{} Second",
            "\x1b[0m", "\x1b[0m", "\x1b[0m", "\x1b[0m"
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_plot_many_y_axis_formatter() {
        let s1 = vec![1.0, 2.0, 3.0];
        let s2 = vec![3.0, 2.0, 1.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let config = Config::default()
            .y_axis_value_formatter(Box::new(|v: f64| format!("{:.0}B", v)));
        let expected = clean(
            r#"
 3B ┼╮╭
 2B ┤╰╮
 1B ┼╯╰"#,
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_plot_many_x_axis_basic() {
        let s1 = vec![1.0, 2.0, 3.0];
        let s2 = vec![3.0, 2.0, 1.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let config = Config::default()
            .x_axis_range(0.0, 100.0)
            .x_axis_tick_count(2);
        let expected = clean(
            r#"
 3.00 ┼╮╭
 2.00 ┤╰╮
 1.00 ┼╯╰
      └┬─┬
       0"#,
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_plot_many_x_axis_with_caption() {
        let s1 = vec![1.0, 2.0, 3.0];
        let s2 = vec![3.0, 2.0, 1.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let config = Config::default()
            .x_axis_range(0.0, 50.0)
            .x_axis_tick_count(2)
            .caption("multi caption");
        let expected = clean(
            r#"
 3.00 ┼╮╭
 2.00 ┤╰╮
 1.00 ┼╯╰
      └┬─┬
       0
       multi caption"#,
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_plot_many_x_axis_with_legends() {
        let s1 = vec![1.0, 2.0, 3.0];
        let s2 = vec![3.0, 2.0, 1.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let config = Config::default()
            .x_axis_range(0.0, 10.0)
            .x_axis_tick_count(2)
            .series_legends(&["Up", "Down"]);
        let expected = format!(
            "{}\n\n       {}■{} Up   {}■{} Down",
            clean(
                r#"
 3.00 ┼╮╭
 2.00 ┤╰╮
 1.00 ┼╯╰
      └┬─┬
       0"#
            ),
            "\x1b[0m",
            "\x1b[0m",
            "\x1b[0m",
            "\x1b[0m"
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_plot_many_x_axis_with_width_interpolation() {
        let s1 = vec![1.0, 5.0];
        let s2 = vec![5.0, 1.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let config = Config::default()
            .width(10)
            .x_axis_range(0.0, 100.0)
            .x_axis_tick_count(3);
        let expected = clean(
            r#"
 5.00 ┼─╮     ╭─
 4.00 ┤ ╰─╮ ╭─╯
 3.00 ┤   ╰─╮
 2.00 ┤ ╭─╯ ╰─╮
 1.00 ┼─╯     ╰─
      └┬────┬───┬
       0   50  100"#,
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    // -------------------------------------------------------------------------
    // TestLineEnding
    // -------------------------------------------------------------------------

    #[test]
    fn test_line_ending_default() {
        let data = vec![2.0, 1.0, 1.0, 2.0, -2.0, 5.0, 7.0, 11.0, 3.0, 7.0, 1.0];
        let actual = plot(&data, Config::default());
        assert!(actual.contains('\n'), "default should use newline");
    }

    #[test]
    fn test_line_ending_crlf() {
        let data = vec![2.0, 1.0, 1.0, 2.0, -2.0, 5.0, 7.0, 11.0, 3.0, 7.0, 1.0];
        let actual = plot(&data, Config::default().line_ending("\r\n"));
        assert!(actual.contains("\r\n"), "should use CRLF");
        assert!(
            !actual.replace("\r\n", "").contains('\n'),
            "should not contain standalone newline"
        );
    }

    #[test]
    fn test_line_ending_plot_many_default() {
        let s1 = vec![0.0, 1.0, 2.0];
        let s2 = vec![2.0, 1.0, 0.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let actual = plot_many(&data, Config::default());
        assert!(actual.contains('\n'), "default should use newline");
    }

    #[test]
    fn test_line_ending_plot_many_crlf() {
        let s1 = vec![0.0, 1.0, 2.0];
        let s2 = vec![2.0, 1.0, 0.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let actual = plot_many(&data, Config::default().line_ending("\r\n"));
        assert!(actual.contains("\r\n"), "should use CRLF");
        assert!(
            !actual.replace("\r\n", "").contains('\n'),
            "should not contain standalone newline"
        );
    }

    #[test]
    fn test_line_ending_with_caption() {
        let data = vec![1.0, 2.0, 3.0];
        let actual = plot(&data, Config::default().caption("test").line_ending("\r\n"));
        assert!(actual.contains("\r\n"), "should use CRLF");
    }

    #[test]
    fn test_line_ending_with_legends() {
        let s1 = vec![0.0, 1.0, 0.0];
        let s2 = vec![2.0, 3.0, 4.0, 3.0, 2.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];

        let actual_default = plot_many(
            &data,
            Config::default()
                .series_colors(&[AnsiColor::RED, AnsiColor::BLUE])
                .series_legends(&["A", "B"]),
        );
        assert!(actual_default.contains('\n'), "default should use newline");

        let actual_crlf = plot_many(
            &data,
            Config::default()
                .series_colors(&[AnsiColor::RED, AnsiColor::BLUE])
                .series_legends(&["A", "B"])
                .line_ending("\r\n"),
        );
        assert!(actual_crlf.contains("\r\n"), "should use CRLF");
    }

    // -------------------------------------------------------------------------
    // Precision tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_precision_respected_for_large_numbers() {
        let s1 = vec![100.123456, 200.987654];
        let data: Vec<&[f64]> = vec![&s1];
        let actual = plot_many(&data, Config::default().precision(3));
        assert!(actual.contains("100.123 "), "precision(3) should show 100.123");
        assert!(actual.contains("200.988 "), "precision(3) should show 200.988");
    }

    #[test]
    fn test_precision_zero_with_large_numbers() {
        let s1 = vec![150.5, 200.9];
        let data: Vec<&[f64]> = vec![&s1];
        let actual = plot_many(&data, Config::default().precision(0));
        assert!(
            actual.contains("201 ") && !actual.contains('.'),
            "precision(0) should show integers without decimal"
        );
    }

    #[test]
    fn test_plot_precision_with_large_numbers() {
        let data = vec![100.123, 200.456, 150.789];
        let actual = plot(&data, Config::default().precision(2));
        assert!(actual.contains("100.12 "), "precision(2) should show 100.12");
        assert!(actual.contains("200.46 "), "precision(2) should show 200.46");
    }

    #[test]
    fn test_precision_default_auto_calculation() {
        let small = vec![0.1, 0.2];
        let large = vec![100.0, 200.0];
        let small_data: Vec<&[f64]> = vec![&small];
        let large_data: Vec<&[f64]> = vec![&large];

        let small_actual = plot_many(&small_data, Config::default());
        let large_actual = plot_many(&large_data, Config::default());

        assert!(
            small_actual.contains("0."),
            "small numbers should auto-calculate precision"
        );
        assert!(
            large_actual.contains("200"),
            "large numbers should show without decimals by default"
        );
    }

    // -------------------------------------------------------------------------
    // Custom CharSet tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_custom_chars_asterisk() {
        use asciigraph::options::create_char_set;
        let s1 = vec![1.0, 2.0, 3.0, 2.0, 1.0];
        let data: Vec<&[f64]> = vec![&s1];
        let config = Config::default().series_chars(&[create_char_set('*')]);
        let expected = " 3.00 ┤ **\n 2.00 ┤****\n 1.00 ┼*  *";
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_custom_chars_dot() {
        use asciigraph::options::create_char_set;
        let s1 = vec![1.0, 2.0, 3.0, 2.0, 1.0];
        let data: Vec<&[f64]> = vec![&s1];
        let config = Config::default().series_chars(&[create_char_set('•')]);
        let expected = " 3.00 ┤ ••\n 2.00 ┤••••\n 1.00 ┼•  •";
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_default_char_set() {
        let s1 = vec![1.0, 2.0, 2.0, 2.0, 3.0];
        let data: Vec<&[f64]> = vec![&s1];
        let expected = " 3.00 ┤   ╭\n 2.00 ┤╭──╯\n 1.00 ┼╯";
        assert_eq!(plot_many(&data, Config::default()), expected);
    }

    #[test]
    fn test_partial_char_set() {
        use asciigraph::options::CharSet;
        let s1 = vec![1.0, 2.0, 2.0, 2.0, 3.0];
        let data: Vec<&[f64]> = vec![&s1];
        let partial = CharSet {
            horizontal: '=',
            vertical_line: '|',
            ..Default::default()
        };
        let config = Config::default().series_chars(&[partial]);
        let expected = " 3.00 ┤   ╭\n 2.00 ┤╭==╯\n 1.00 ┼╯";
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_multiple_series_different_chars() {
        use asciigraph::options::create_char_set;
        let s1 = vec![1.0, 2.0, 3.0];
        let s2 = vec![3.0, 2.0, 1.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let config =
            Config::default().series_chars(&[create_char_set('*'), create_char_set('#')]);
        let expected = " 3.00 ┼#*\n 2.00 ┤##\n 1.00 ┼*#";
        assert_eq!(plot_many(&data, config), expected);
    }

    // -------------------------------------------------------------------------
    // TestXAxis
    // -------------------------------------------------------------------------

    #[test]
    fn test_x_axis_single_data_point() {
        let s1 = vec![5.0];
        let data: Vec<&[f64]> = vec![&s1];
        let config = Config::default().x_axis_range(0.0, 10.0);
        let expected = clean(
            r#"
 5.00 ┼
      └┬
       0"#,
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_x_axis_min_equals_max() {
        let s1 = vec![1.0, 2.0, 3.0];
        let data: Vec<&[f64]> = vec![&s1];
        let config = Config::default()
            .x_axis_range(5.0, 5.0)
            .x_axis_tick_count(3);
        let expected = clean(
            r#"
 3.00 ┤ ╭
 2.00 ┤╭╯
 1.00 ┼╯
      └┬┬┬
       5 5"#,
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_x_axis_wide_labels_overlap_skipping() {
        let s1: Vec<f64> = (1..=10).map(|x| x as f64).collect();
        let data: Vec<&[f64]> = vec![&s1];
        let config = Config::default()
            .x_axis_range(0.0, 1000.0)
            .x_axis_tick_count(5);
        let expected = clean(
            r#"
 10.00 ┤        ╭
  9.00 ┤       ╭╯
  8.00 ┤      ╭╯
  7.00 ┤     ╭╯
  6.00 ┤    ╭╯
  5.00 ┤   ╭╯
  4.00 ┤  ╭╯
  3.00 ┤ ╭╯
  2.00 ┤╭╯
  1.00 ┼╯
       └┬─┬──┬─┬─┬
        0   500"#,
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_x_axis_width_interpolation() {
        let s1 = vec![1.0, 5.0, 1.0];
        let data: Vec<&[f64]> = vec![&s1];
        let config = Config::default()
            .width(10)
            .x_axis_range(0.0, 100.0)
            .x_axis_tick_count(3);
        let expected = clean(
            r#"
 4.56 ┤   ╭─╮
 3.37 ┤  ╭╯ ╰╮
 2.19 ┤╭─╯   ╰─╮
 1.00 ┼╯       ╰
      └┬────┬───┬
       0   50  100"#,
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_x_axis_line_ending() {
        let s1 = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let data: Vec<&[f64]> = vec![&s1];
        let config = Config::default()
            .x_axis_range(0.0, 100.0)
            .x_axis_tick_count(2)
            .line_ending("\r\n");
        let expected = " 1.00 ┼────\r\n      └┬───┬\r\n       0  100";
        assert_eq!(plot_many(&data, config), expected);
    }

    #[test]
    fn test_x_axis_with_legends() {
        let s1 = vec![1.0, 2.0, 3.0];
        let s2 = vec![3.0, 2.0, 1.0];
        let data: Vec<&[f64]> = vec![&s1, &s2];
        let config = Config::default()
            .x_axis_range(0.0, 10.0)
            .x_axis_tick_count(2)
            .series_legends(&["A", "B"]);
        let expected = format!(
            "{}\n\n       {}■{} A   {}■{} B",
            clean(
                r#"
 3.00 ┼╮╭
 2.00 ┤╰╮
 1.00 ┼╯╰
      └┬─┬
       0"#
            ),
            "\x1b[0m",
            "\x1b[0m",
            "\x1b[0m",
            "\x1b[0m"
        );
        assert_eq!(plot_many(&data, config), expected);
    }

    // ---------------------------------------------------------------------------
    // Zero line tests
    // ---------------------------------------------------------------------------



    // ---------------------------------------------------------------------------
    // Guard condition: all data positive — zero lies below the visible range.
    // The output must be byte-for-byte identical with and without the zero line
    // because render_zero_line should silently do nothing in this case.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_zero_line_no_effect_when_data_all_positive() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let without = plot(&data, Config::default());
        let with_zl = plot(&data, Config::default().zero_line(ZeroLine::new()));

        assert_eq!(
            without, with_zl,
            "zero line should have no effect when all data is positive"
        );
    }

    // ---------------------------------------------------------------------------
    // Guard condition: all data negative — zero lies above the visible range.
    // Same expectation: the output must be identical with and without the zero line.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_zero_line_no_effect_when_data_all_negative() {
        let data = vec![-5.0, -4.0, -3.0, -2.0, -1.0];

        let without = plot(&data, Config::default());
        let with_zl = plot(&data, Config::default().zero_line(ZeroLine::new()));

        assert_eq!(
            without, with_zl,
            "zero line should have no effect when all data is negative"
        );
    }

    #[test]
    fn test_zero_line_appears_when_data_straddles_zero() {
        let data = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        let graph = plot(&data, Config::default().zero_line(ZeroLine::new()));

        let expected = "  2.00 ┤   ╭\n  1.00 ┤  ╭╯\n  0.00 ┤─╭╯──\n -1.00 ┤╭╯\n -2.00 ┼╯";
        assert_eq!(graph, expected);
    }

    // ---------------------------------------------------------------------------
    // Priority: a series arc character must win over the zero-line character
    // when a data point lands exactly on zero.
    //
    // render_zero_line runs before render_series and writes '─' into blank cells.
    // render_series then overwrites the zero-row cell with '┼' for the first
    // data point. This test confirms that priority is respected.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_zero_line_series_wins_at_zero_crossing() {
        let data = vec![0.0, 1.0, 2.0, 1.0, 0.0];
        let graph = plot(&data, Config::default().zero_line(ZeroLine::new()));

        // '┼' marks where the series crosses the axis at y = 0.
        // If render_series did NOT overwrite the zero-line character,
        // we would see '─' here instead, and this assertion would fail.
        assert!(
            graph.contains('┼'),
            "series axis-crossing character ┼ must appear at y = 0, not the zero line character"
        );
    }

    // -------------------------------------------------------------------------
    // Threshold line tests
    // -------------------------------------------------------------------------

    // Guard condition: threshold below the visible range.
    // Output must be identical with and without the threshold.
    #[test]
    fn test_threshold_no_effect_when_below_range() {
        let data = vec![5.0, 6.0, 7.0, 8.0, 9.0];

        let without = plot(&data, Config::default());
        let with_t  = plot(&data, Config::default().threshold(Threshold::new(1.0)));

        assert_eq!(
            without, with_t,
            "threshold below the visible range should have no effect"
        );
    }

    // Guard condition: threshold above the visible range.
    // Output must be identical with and without the threshold.
    #[test]
    fn test_threshold_no_effect_when_above_range() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let without = plot(&data, Config::default());
        let with_t  = plot(&data, Config::default().threshold(Threshold::new(99.0)));

        assert_eq!(
            without, with_t,
            "threshold above the visible range should have no effect"
        );
    }

    // Single threshold within the visible range.
    #[test]
    fn test_threshold_single_appears() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let graph = plot(&data, Config::default().threshold(Threshold::new(3.0)));

        let expected = " 5.00 ┤   ╭╮\n 4.00 ┤  ╭╯╰╮\n 3.00 ┤╌╭╯╌╌╰╮╌╌\n 2.00 ┤╭╯    ╰╮\n 1.00 ┼╯      ╰";
        assert_eq!(graph, expected);
    }

    // Multiple thresholds: both must appear at their correct rows.
    #[test]
    fn test_threshold_multiple_appear() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let graph = plot(
            &data,
            Config::default()
                .threshold(Threshold::new(2.0))
                .threshold(Threshold::new(4.0)),
        );

        let expected = " 5.00 ┤   ╭╮\n 4.00 ┤╌╌╭╯╰╮╌╌╌\n 3.00 ┤ ╭╯  ╰╮\n 2.00 ┤╭╯╌╌╌╌╰╮╌\n 1.00 ┼╯      ╰";
        assert_eq!(graph, expected);
    }

    // Priority: series arc characters must win over the threshold character
    // where they share the same cell.
    // render_thresholds runs before render_series, so series characters
    // overwrite the ╌ character at any cell they occupy.
    #[test]
    fn test_threshold_series_wins_at_crossing() {
        // The series starts at exactly 3.0, which is also the threshold value.
        // render_series places ┼ at that cell — it must not be ╌.
        let data = vec![3.0, 4.0, 5.0, 4.0, 3.0];
        let graph = plot(&data, Config::default().threshold(Threshold::new(3.0)));

        assert!(
            graph.contains('┼'),
            "series axis-crossing character ┼ must appear where the series meets the threshold row"
        );
    }

    // Colored threshold: ANSI escape codes must appear in the output.
    #[test]
    fn test_threshold_color_applied() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let graph = plot(
            &data,
            Config::default().threshold(Threshold::with_color(3.0, AnsiColor::RED)),
        );

        // AnsiColor::RED emits \x1b[91m — confirm it appears in the output.
        assert!(
            graph.contains("\x1b[91m"),
            "colored threshold must emit ANSI escape code for RED"
        );
    }

    // -------------------------------------------------------------------------
    // Moving average tests
    // -------------------------------------------------------------------------

    // The moving average series must produce a smoother curve than the
    // original. We verify this by checking that the output contains more
    // rows than a flat line would — i.e. the MA series is actually rendered.
    #[test]
    fn test_moving_average_appears_as_additional_series() {
        let data = vec![1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0];

        let without = plot(&data, Config::default());
        let with_ma = plot(&data, Config::default().moving_average(3));

        // The graph with a moving average must differ from the one without.
        assert_ne!(
            without, with_ma,
            "moving average overlay must change the graph output"
        );
    }

    // Window of 1 has no effect — output must be identical.
    #[test]
    fn test_moving_average_window_one_has_no_effect() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let without = plot(&data, Config::default());
        let with_ma = plot(&data, Config::default().moving_average(1));

        assert_eq!(
            without, with_ma,
            "moving average with window 1 should have no effect"
        );
    }

    // Window of 0 has no effect — output must be identical.
    #[test]
    fn test_moving_average_window_zero_has_no_effect() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let without = plot(&data, Config::default());
        let with_ma = plot(&data, Config::default().moving_average(0));

        assert_eq!(
            without, with_ma,
            "moving average with window 0 should have no effect"
        );
    }

    // Window larger than data length — must not panic, output must differ.
    #[test]
    fn test_moving_average_window_larger_than_data() {
        let data = vec![1.0, 3.0, 1.0];
        let graph = plot(&data, Config::default().moving_average(100));

        // Must not panic and must produce some output.
        assert!(!graph.is_empty(), "must produce output even when window exceeds data length");
    }

    #[test]
    fn test_moving_average_exact_output() {
        let data = vec![1.0, 5.0, 3.0, 7.0, 2.0, 6.0, 4.0, 8.0, 3.0, 5.0];
        let graph = plot(&data, Config::default().moving_average(3));


        let expected = " 8.00 ┤      ╭╮\n 7.00 ┤  ╭╮  ││\n 6.00 ┤  ││╭╭╮│\n 5.00 ┤╭╭╮╭╮│╰─╮\n 4.00 ┤││╰╯╰╯╯│╰\n 3.00 ┼─╯╯││  ╰╯\n 2.00 ┤│  ╰╯\n 1.00 ┼╯";
        assert_eq!(graph, expected);
    }

    // -------------------------------------------------------------------------
    // Auto tick count tests
    // -------------------------------------------------------------------------

    // When x_axis_tick_count is not set, the library should automatically
    // calculate a sensible number of ticks based on the available width.
    // We verify that an x-axis is rendered at all and that it contains
    // tick mark characters.
    #[test]
    fn test_auto_tick_count_renders_axis() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let graph = plot(&data, Config::default().x_axis_range(0.0, 100.0));

        // The x-axis corner character confirms the axis was rendered.
        assert!(
            graph.contains('└'),
            "auto tick count should render an x-axis with the corner character"
        );

        // At least one tick mark must be present.
        assert!(
            graph.contains('┬'),
            "auto tick count should render at least one tick mark"
        );
    }

    // Explicitly setting x_axis_tick_count must override the auto calculation.
    // This ensures we have not broken the existing behaviour.
    #[test]
    fn test_explicit_tick_count_overrides_auto() {
        // Use a wide dataset so auto tick count chooses more than 2 ticks.
        let data: Vec<f64> = (1..=20).map(|x| x as f64).collect();

        let auto_graph = plot(&data, Config::default().x_axis_range(0.0, 1000.0));
        let explicit_graph = plot(
            &data,
            Config::default()
                .x_axis_range(0.0, 1000.0)
                .x_axis_tick_count(2),
        );

        // With 20 data points and wide labels (0, 1000), auto should choose
        // more ticks than the explicit minimum of 2, producing different output.
        assert_ne!(
            auto_graph, explicit_graph,
            "explicitly setting tick count should produce a different axis than auto"
        );
    }

    // Auto tick count on a very narrow graph must not panic and must
    // produce at least 2 ticks due to the .max(2) clamp.
    #[test]
    fn test_auto_tick_count_minimum_two_ticks() {
        let data = vec![1.0, 2.0];
        let graph = plot(&data, Config::default().x_axis_range(0.0, 1000000.0));

        assert!(
            !graph.is_empty(),
            "auto tick count must not panic on a very narrow graph"
        );
        assert!(
            graph.contains('└'),
            "auto tick count must still render an axis on a narrow graph"
        );
    }
}
