#[macro_use]
extern crate afl;

use std::process::{Command, Stdio};
use std::io::Write;

fn main() {
    fuzz!(|data: &[u8]| {
        let mut child = Command::new("./target/debug/scanner")
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to spawn scanner");

        if let Some(stdin) = child.stdin.as_mut() {
            let _ = stdin.write_all(data);
        }

        let _ = child.wait();
    });
}
