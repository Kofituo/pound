use crossterm::event::*;
use crossterm::terminal::ClearType; /* add import */
use crossterm::{event, execute, terminal}; /* add import */
use std::io::stdout;
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

struct Output;

impl Output {
    fn new() -> Self {
        Self
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))
    }

    fn refresh_screen(&self) -> crossterm::Result<()> {
        Self::clear_screen()
    }
}

struct Reader;

impl Reader {
    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

struct Editor {
    reader: Reader,
    output: Output, /* add this line*/
}

impl Editor {
    fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(), /* add this line */
        }
    }

    fn process_keypress(&self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    fn run(&self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?; /* add this line*/
        self.process_keypress()
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
