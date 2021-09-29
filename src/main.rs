use crossterm::terminal; /* add this line */
use std::io;
use std::io::Read;
fn main() {
    terminal::enable_raw_mode().expect("Could not turn on Raw mode"); /* add this line */
    let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {}
}
