use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};
use std::time::Duration; /* add this line */

const QUIT: u8 = 1;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

fn editor_process_key(event: &KeyEvent) -> u8 {
    let has_modifiers = !event.modifiers.is_empty();
    match event.code {
        KeyCode::Char(val) => {
            if !has_modifiers && val == 'q' {
                return QUIT;
            }
        }
        _ => {}
    }
    0
}

fn main() {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    loop {
        /* modify */
        if event::poll(Duration::from_millis(500)).expect("Error occurred") {
            let return_value = match event::read().expect("Failed to read line") {
                Event::Key(event) => {
                    println!("{:?}\r", event);
                    editor_process_key(&event)
                }
                _ => 0,
            };
            if return_value == QUIT {
                break;
            }
        } else {
            println!("No input yet\r");
        }
        /* end */
    }
}
