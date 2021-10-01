use crossterm::event::{Event, KeyCode, KeyEvent}; /* modify */
use crossterm::{event, terminal};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

fn main() {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    /* add the following */
    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::NONE,
                } => break,
                _ => {
                    //todo
                }
            }
            println!("{:?}\r", event);
        };
    }
    /* end */
    /* comment out the following
    let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("Failed to read") == 1 && buf != [b'q'] {
        let character = buf[0] as char;
        if character.is_control() {
            println!("{}\r", character as u8)
        } else {
            println!("{}\r", character)
        }
    }*/
}
