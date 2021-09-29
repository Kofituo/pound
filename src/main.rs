use crossterm::event::{Event, KeyCode, KeyEvent}; /* modify */
use crossterm::{event, terminal};
//use std::io; /* comment out this line */
//use std::io::Read; /* comment out this line */
const QUIT: u8 = 1; /* add this line*/

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
            //configure 'q' to exit program
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
    /* add the following*/
    loop {
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
    }
    /*end*/
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
