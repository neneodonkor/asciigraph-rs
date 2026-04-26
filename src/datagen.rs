use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    // Launch asciigraph as a child process with its stdin piped
    let mut child = Command::new("asciigraph")
        .args(["-r", "-h", "10", "-w", "40", "-c", "realtime data"])
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to start asciigraph — make sure it is installed");

    // Get a handle to the child process's stdin
    let stdin = child.stdin.as_mut().expect("failed to open stdin");

    for _ in 0..1000 {
        let val = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_millis() % 90 + 10;

        // Write directly to asciigraph's stdin and flush immediately
        // This bypasses Windows pipe buffering entirely because we own
        // the pipe handle directly rather than going through a shell pipeline
        writeln!(stdin, "{}", val).unwrap();
        stdin.flush().unwrap();

        thread::sleep(Duration::from_millis(200));
    }

    child.wait().unwrap();
}