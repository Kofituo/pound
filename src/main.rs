use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, terminal};
use crossterm::{execute, Command};
use std::fmt::Formatter;
use std::io::{stdout, Write as IO_WRITE};
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        Output::clear_screen().expect("Clear screen failed");
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

struct Reader {
    key: Option<KeyEvent>,
}

/* add the following*/
impl Reader {
    fn new() -> Self {
        Self { key: None }
    }

    fn read_key(&mut self) -> crossterm::Result<()> {
        loop {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(event) = event::read()? {
                    self.key = Some(event);
                    return Ok(());
                };
            }
        }
    }

    fn key_pressed(&mut self) -> crossterm::Result<KeyEvent> {
        self.read_key()?;
        std::fs::write("test.txt", format!("as {:?}", self.key.unwrap())).unwrap();
        Ok(self.key.unwrap())
    }
}

struct EditorContents {
    content: String,
}

impl EditorContents {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
}

impl std::fmt::Write for EditorContents {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.push_str(s);
        Ok(())
    }
}

impl std::fmt::Display for EditorContents {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl EditorContents {
    fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    fn reset(&mut self) {
        self.content.clear()
    }

    fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

macro_rules! write_ansi {
    ($dst:expr, $($arg:expr),*) => {
        {
            $( $arg.write_ansi(&mut $dst).unwrap();)*
        }
    };
}

struct Output {
    win_size: (u16, u16),
    editor_contents: EditorContents,
}

const VERSION: &str = "0.0.1";

impl Output {
    fn new() -> Self {
        Self {
            win_size: terminal::size().unwrap(),
            editor_contents: EditorContents::new(),
        }
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), crossterm::cursor::MoveTo(0, 0))
    }

    fn draw_rows(&mut self) {
        let number_of_rows = self.win_size.1;
        let number_of_columns = self.win_size.0 as usize;
        for i in 0..number_of_rows {
            if i == number_of_rows / 3 {
                let mut welcome_msg = format!("Welcome to Pound editor -- version {}", VERSION);
                if welcome_msg.len() > number_of_columns {
                    welcome_msg.truncate(number_of_columns)
                }
                let mut padding = (number_of_columns - welcome_msg.len()) / 2;
                if padding != 0 {
                    self.editor_contents.push('~');
                    padding -= 1
                }
                (0..padding).for_each(|_| self.editor_contents.push(' '));
                self.editor_contents.push_str(&welcome_msg);
            } else {
                self.editor_contents.push('~');
            }
            write_ansi!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            );
            if i < number_of_rows - 1 {
                self.editor_contents.push_str("\r\n");
            }
        }
    }

    fn editor_refresh_screen(
        &mut self,
        cursor_controller: &CursorController,
    ) -> crossterm::Result<()> {
        write_ansi!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0));
        self.draw_rows();
        write_ansi!(
            self.editor_contents,
            cursor::MoveTo(cursor_controller.cursor_x, cursor_controller.cursor_y),
            cursor::Show
        );
        crossterm::queue!(self.editor_contents, Print(&self.editor_contents));
        self.editor_contents.reset();
        Ok(())
    }
}

struct CursorController {
    cursor_x: u16,
    cursor_y: u16,
}

impl CursorController {
    fn new() -> Self {
        Self {
            cursor_y: 0,
            cursor_x: 0,
        }
    }

    fn move_cursor(&mut self, direction: KeyCode, win_size: (u16, u16)) {
        match direction {
            KeyCode::Left => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1
                }
            }
            KeyCode::Right => {
                if self.cursor_x != win_size.0 {
                    self.cursor_x += 1
                }
            }
            KeyCode::Up => {
                if self.cursor_y != 0 {
                    self.cursor_y -= 1
                }
            }
            KeyCode::Down => {
                if self.cursor_y != win_size.1 {
                    self.cursor_y += 1
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
struct Editor {
    reader: Reader,
    output: Output,
    cursor_controller: CursorController,
}

impl Editor {
    fn new() -> Self {
        Self {
            reader: Reader::new(),
            output: Output::new(),
            cursor_controller: CursorController::new(),
        }
    }
    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.reader.key_pressed()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
            } => return Ok(false),
            KeyEvent {
                code: val @ (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right),
                modifiers: event::KeyModifiers::NONE,
            } => self
                .cursor_controller
                .move_cursor(val, self.output.win_size),
            _ => {}
        }
        Ok(true)
    }

    fn run(&mut self) -> crossterm::Result<bool> {
        self.output.editor_refresh_screen(&self.cursor_controller)?;
        self.process_keypress()
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
