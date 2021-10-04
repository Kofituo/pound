use crossterm::event::*;
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, terminal};
use std::io::stdout;
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        Output::clear_screen().expect("error");
    }
}

struct Output {
    win_size:(usize,usize)
};

impl Output {
    fn new() -> Self {
        let win_size =terminal::size().and_then(|it|)
        Self {
            win_size:
        }
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    /* add this function */
    fn draw_rows(&self) {
        for _ in 0..24 {
            println!("~\r");
        }
    }

    fn refresh_screen(&self) -> crossterm::Result<()> {
        Self::clear_screen()?;
        /* add the following lines*/
        self.draw_rows();
        execute!(stdout(), cursor::MoveTo(0, 0))
        /* end */
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
    output: Output,
}

impl Editor {
    fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
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
        self.output.refresh_screen()?;
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
