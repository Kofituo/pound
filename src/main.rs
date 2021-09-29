use crossterm::terminal; /* add this line */
use std::io;
use std::io::Read;
fn main() {
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {}
    terminal::disable_raw_mode().expect("Could not turn off raw mode"); /* add this line */
}
