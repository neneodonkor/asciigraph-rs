use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    // Print 5 lines so we have something to clear
    for i in 1..=5 {
        println!("Line {}", i);
    }

    thread::sleep(Duration::from_secs(2));

    // Try to clear and reset
    print!("\x1b[H\x1b[J");
    io::stdout().flush().unwrap();

    println!("This should appear at the top after a clean clear");

    thread::sleep(Duration::from_secs(2));
}