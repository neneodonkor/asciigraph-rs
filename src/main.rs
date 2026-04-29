use std::io::{self, BufRead, Write};
use std::time::{Duration, Instant};
// This is the main entry point into the CLI.
use clap::{Parser};
use asciigraph;
use asciigraph::{AnsiColor, CharSet, create_char_set, plot_many};

#[derive(Parser, Debug)]
#[command(name = "asciigraph", disable_help_flag = true)]
#[command(about = "Plot data from stdin as an ASCII graph")]
struct Args {
    #[arg(short = 'h', long, default_value_t = 0)]
    height: usize,

    #[arg(short = 'w', long, default_value_t = 0)]
    width: usize,

    #[arg(short = 'o', long, default_value_t = 3)]
    offset: usize,

    #[arg(short = 'p', long, default_value_t = 2)]
    precision: usize,

    #[arg(short = 'c', long, default_value_t = String::new())]
    caption: String,

    #[arg(short = 'r', long, default_value_t = false)]
    realtime: bool,

    #[arg(short = 'b', long="buffer", default_value_t = 0)]
    realtime_buffer: usize,

    #[arg(short = 'f', long, default_value_t = 24.0)]
    fps: f64,

    #[arg(long = "sc", default_value_t = String::new())]
    series_colors: String,  // comma-separated, parsed later

    #[arg(long = "sl", default_value_t = String::new())]
    series_legends: String,  // comma-separated, parsed later

    #[arg(long = "cc", default_value_t = String::new())]
    caption_color: String,  // color name, parsed later

    #[arg(long = "ac", default_value_t = String::new())]
    axis_color: String,

    #[arg(long = "lc", default_value_t = String::new())]
    label_color: String,

    #[arg(long = "lb", default_value_t = f64::INFINITY)]
    lower_bound: f64,

    #[arg(long = "ub", default_value_t = f64::NEG_INFINITY)]
    upper_bound: f64,

    #[arg(short = 'd', long, default_value_t = String::from(","))]
    delimiter: String,

    #[arg(long = "sn", default_value_t = 1)]
    series_num: usize,

    #[arg(short = 'x', long = "custom-char", default_value_t = String::new())]
    custom_char: String,

    #[arg(long = "xmin", default_value_t = f64::NAN)]
    x_axis_min: f64,

    #[arg(long = "xmax", default_value_t = f64::NAN)]
    x_axis_max: f64,

    #[arg(long = "xt", default_value_t = 5)]
    x_axis_ticks: usize,

    // add help back manually
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,
}

fn main() {
    let mut args = Args::parse();

    let mut char_sets: Vec<CharSet> = Vec::new(); // Same as seriesCharsOptions in Go.
    let mut series: Vec<Vec<f64>> = vec![vec![]; args.series_num];
    let custom_char = args.custom_char.clone();

    if !custom_char.is_empty() {
        let chars = custom_char.split(',');

        for c in chars {
            // Handle cases where the user enters an empty character.
            let trimmed = c.trim();
            match trimmed.parse::<char>() {
                Ok(ch) => char_sets.push(create_char_set(ch)),
                Err(_) => {
                    eprintln!("invalid character: {}", trimmed);
                    std::process::exit(1);
                }
            }
        }
    }

    if args.realtime && args.realtime_buffer == 0 {
        args.realtime_buffer = args.width;
    }

    let x_axis_enabled = !args.x_axis_min.is_nan() && !args.x_axis_max.is_nan();

    // 1 -------------------------------------------------------------------------------------------

    // s := bufio.NewScanner(os.Stdin)
    // s.Split(bufio.ScanLines)
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut next_flush_time = Instant::now();
    let flush_interval = Duration::from_secs_f64(1.0/ args.fps);

    for line in lines {
        let line = line.unwrap();
        let mut points: Vec<_> = line.split(&args.delimiter).collect();

        if points.len() < args.series_num {
            eprintln!("number of series in the input data stream is less than the specified series number");
            std::process::exit(1);
        } else if points.len() > args.series_num {
            // points = points[..args.series_num].to_owned();
            // this is a simply way of truncating an array.
            points.truncate(args.series_num);
        }

        for (i, point) in points.iter().enumerate() {
            let p = match point.trim().parse::<f64>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("ignore {:?}: cannot parse value", point.trim());
                    f64::NAN
                }
            };
            series[i].push(p);
        }

        // 2 ---------------------------------------------------------------------------------------

        if args.realtime {
            if args.realtime_buffer > 0 && series[0].len() > args.realtime_buffer {
                for i in 0..series.len() {
                    let series_length = series[i].len();
                    series[i] = series[i].split_off(series_length - args.realtime_buffer);
                }
            }

            if Instant::now() >= next_flush_time {
                let series_copy = series.iter().map(|s| s.to_vec()).collect::<Vec<_>>();
                let colors_series: Vec<AnsiColor> = parse_colors(&args.series_colors)
                    .unwrap_or_default();

                let cc = parse_color(&args.caption_color).unwrap_or(AnsiColor::default());
                let ac = parse_color(&args.axis_color).unwrap_or(AnsiColor::default());
                let lc = parse_color(&args.label_color).unwrap_or(AnsiColor::default());

                let legends: Vec<&str> = if args.series_legends.is_empty() {
                    vec![]
                } else {
                    args.series_legends.split(',').map(|s| s.trim()).collect()
                };


                let mut config = asciigraph::Config::default();
                config = config.height(args.height)
                    .width(args.width)
                    .offset(args.offset)
                    .precision(args.precision)
                    .caption(args.caption.as_str())
                    .series_colors(&colors_series)
                    .series_legends(&legends)
                    .caption_color(cc)
                    .axis_color(ac)
                    .label_color(lc)
                    .lower_bound(args.lower_bound)
                    .upper_bound(args.upper_bound);

                if !char_sets.is_empty() {
                    config = config.series_chars(&char_sets);
                }

                if x_axis_enabled {
                    config = config.x_axis_range(args.x_axis_min, args.x_axis_max);
                    if args.x_axis_ticks > 0 {
                        config = config.x_axis_tick_count(args.x_axis_ticks);
                    }
                }

                let series_refs: Vec<&[f64]> = series_copy.iter().map(|s| s.as_slice()).collect();
                let plot = plot_many(&series_refs, config);
                // clear();
                // println!("{}", plot);
                // io::stdout().flush().unwrap();
                // next_flush_time = Instant::now() + flush_interval;
                let output = format!("\x1b[H\x1b[J{}", plot);
                print!("{}", output);
                io::stdout().flush().unwrap();
                next_flush_time = Instant::now() + flush_interval;
            }
        }
    }

    if !args.realtime {
        if series[0].is_empty() {
            eprintln!("No data!");
            std::process::exit(1);
        }

        let colors_series: Vec<AnsiColor> = parse_colors(&args.series_colors)
            .unwrap_or_default();

        let series_copy = series.iter().map(|s| s.to_vec()).collect::<Vec<_>>();
        let cc = parse_color(&args.caption_color).unwrap_or(AnsiColor::default());
        let ac = parse_color(&args.axis_color).unwrap_or(AnsiColor::default());
        let lc = parse_color(&args.label_color).unwrap_or(AnsiColor::default());

        let legends: Vec<&str> = if args.series_legends.is_empty() {
            vec![]
        } else {
            args.series_legends.split(',').map(|s| s.trim()).collect()
        };


        let mut config = asciigraph::Config::default();
        config = config.height(args.height)
            .width(args.width)
            .offset(args.offset)
            .precision(args.precision)
            .caption(args.caption.as_str())
            .series_colors(&colors_series)
            .series_legends(&legends)
            .caption_color(cc)
            .axis_color(ac)
            .label_color(lc)
            .lower_bound(args.lower_bound)
            .upper_bound(args.upper_bound);

        if !char_sets.is_empty() {
            config = config.series_chars(&char_sets);
        }

        if x_axis_enabled {
            config = config.x_axis_range(args.x_axis_min, args.x_axis_max);
            if args.x_axis_ticks > 0 {
                config = config.x_axis_tick_count(args.x_axis_ticks);
            }
        }

        let series_refs: Vec<&[f64]> = series_copy.iter().map(|s| s.as_slice()).collect();
        let plot = plot_many(&series_refs, config);

        println!("{plot}");
    }
}

fn parse_colors(colors: &str) -> Option<Vec<AnsiColor>> {
    let color_list = colors.split(',');
    let mut parsed_colors: Vec<AnsiColor> = Vec::new();

    for color in color_list {
        match parse_color(color.trim()) {
            Some(c) => parsed_colors.push(c),
            None => return None
        }
    }

    Some(parsed_colors)
}

fn parse_color(color: &str) -> Option<AnsiColor> {
    AnsiColor::get_ansi_color(color)
}
