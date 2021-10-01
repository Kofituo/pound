use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};
use std::time::Duration; /* add this line */

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

fn main() {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    loop {
        if event::poll(Duration::from_millis(500)).expect("Error") {
            /* add this line */
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
        /* add the following*/
        } else {
            println!("No input yet\r");
        }
        /* end */
    }
}
