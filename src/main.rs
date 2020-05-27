extern crate base64;

use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    let osc_buffer = format!("\x1b]52;0;{}\x07", base64::encode(buffer.as_bytes()));
    std::io::stdout().write_all(osc_buffer.as_bytes()).unwrap();
    std::io::stdout().flush().unwrap();
    Ok(())
}
