use asciigraph::Config;

fn main() {
    // Build a config with several options set.
    let config = Config::default()
        .height(10)
        .width(30)
        .caption("Serialization example")
        .precision(2);

    // Serialize to JSON.
    let json = serde_json::to_string_pretty(&config)
        .expect("serialization failed");

    println!("Serialized Config:\n{}\n", json);

    // Deserialize back from JSON.
    let restored: Config = serde_json::from_str(&json)
        .expect("deserialization failed");

    println!("Restored height:    {}", restored.height);
    println!("Restored width:     {}", restored.width);
    println!("Restored caption:   {}", restored.caption);
    println!("Restored precision: {:?}", restored.precision);

    // Output:
    // Serialized Config:
    // {
    //     "width": 30,
    //     "height": 10,
    //     "lower_bound": null,
    //     "upper_bound": null,
    //     "offset": 3,
    //     "caption": "Serialization example",
    //     "precision": 2,
    //     "caption_color": 0,
    //     "axis_color": 0,
    //     "label_color": 0,
    //     "series_colors": [],
    //     "series_legends": [],
    //     "line_ending": "\n",
    //     "series_chars": [],
    //     "x_axis_tick_count": 0,
    //     "x_axis_range": null,
    //     "zero_line": null,
    //     "thresholds": [],
    //     "moving_average_window": null,
    //     "x_axis_label": null,
    //     "y_axis_label": null
    // }
    //
    // Restored height:    10
    // Restored width:     30
    // Restored caption:   Serialization example
    // Restored precision: Some(2)
}