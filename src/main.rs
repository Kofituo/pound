use crossterm::event::*;
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, terminal};
use crossterm::{execute, queue};
use std::cmp;
use std::fs::OpenOptions;
use std::io::{stdout, Write};
use std::path::Path;
use std::time::{Duration, Instant};

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

impl Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Ok(0),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

impl EditorContents {
    fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

struct Output {
    win_size: (usize, usize),
    editor_contents: EditorContents,
    editor_rows: EditorRows,
    cursor_controller: CursorController,
}

const VERSION: &str = "0.0.1";

impl Output {
    fn new() -> Self {
        let editor_rows = EditorRows::new();
        let number_of_rows = editor_rows.number_of_rows();
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            editor_contents: EditorContents::new(),
            editor_rows,
            cursor_controller: CursorController::new(number_of_rows, win_size),
        }
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;
        let number_of_rows = self.editor_rows.number_of_rows();
        for i in 0..screen_rows {
            let current_file_row = i + self.cursor_controller.row_offset;
            if current_file_row >= number_of_rows {
                if number_of_rows == 0 && i == screen_rows / 3 {
                    let mut welcome_msg = format!("Welcome to Pound editor -- version {}", VERSION);
                    if welcome_msg.len() > screen_columns {
                        welcome_msg.truncate(screen_columns)
                    }
                    let mut padding = (screen_columns - welcome_msg.len()) / 2;
                    if padding != 0 {
                        self.editor_contents.push('~');
                        padding -= 1
                    }
                    (0..padding).for_each(|_| self.editor_contents.push(' '));
                    self.editor_contents.push_str(&welcome_msg);
                } else {
                    self.editor_contents.push('~');
                }
            } else {
                let row = self.editor_rows.get_render_row(current_file_row);
                let column_offset = self.cursor_controller.column_offset;
                let len = if row.len() < column_offset {
                    0
                } else {
                    cmp::min(row.len() - column_offset, screen_columns)
                };
                let start = if len == 0 { 0 } else { column_offset };
                self.editor_contents.push_str(&row[start..start + len]);
            }
            queue!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            )
            .expect("Error");
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n");
            }
        }
    }

    fn editor_refresh_screen(&mut self) -> crossterm::Result<()> {
        let start = Instant::now();
        self.cursor_controller.scroll();
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        let cursor_x = self.cursor_controller.cursor_x - self.cursor_controller.column_offset;
        let cursor_y = self.cursor_controller.cursor_y - self.cursor_controller.row_offset;
        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.editor_contents.flush()?;

        std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("other.txt")
            .unwrap()
            .write_all(format!("time {:?}\n", start.elapsed()).as_ref())
    }

    fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller
            .move_cursor(direction, &self.editor_rows)
    }
}

struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
    number_of_rows: usize,
    row_offset: usize,
    screen_size: (usize, usize),
    column_offset: usize,
}

impl CursorController {
    fn new(number_of_rows: usize, screen_size: (usize, usize)) -> Self {
        Self {
            cursor_y: 0,
            cursor_x: 0,
            number_of_rows,
            row_offset: 0,
            screen_size,
            column_offset: 0,
        }
    }

    fn scroll(&mut self) {
        self.row_offset = cmp::min(self.row_offset, self.cursor_y);
        if self.cursor_y >= self.row_offset + self.screen_size.1 {
            self.row_offset = self.cursor_y - self.screen_size.1 + 1;
        }
        self.column_offset = cmp::min(self.column_offset, self.cursor_x);
        if self.cursor_x >= self.column_offset + self.screen_size.0 {
            self.column_offset = self.cursor_x - self.screen_size.0 + 1;
        }
    }

    fn move_cursor(&mut self, direction: KeyCode, editor_rows: &EditorRows) {
        let current_row = if self.cursor_y < self.number_of_rows {
            Some(editor_rows.get_row(self.cursor_y))
        } else {
            None
        };
        match direction {
            KeyCode::Left => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = editor_rows.get_row(self.cursor_y).len();
                }
            }
            KeyCode::Right => {
                if let Some(row) = current_row {
                    if self.cursor_x == row.len() {
                        self.cursor_y += 1;
                        self.cursor_x = 0
                    } else {
                        std::fs::write("other_scroll.txt", "assert").unwrap();
                        assert!(self.cursor_x < row.len());
                        self.cursor_x += 1;
                    }
                }
            }
            KeyCode::Up => {
                if self.cursor_y != 0 {
                    self.cursor_y -= 1
                }
            }
            KeyCode::Down => {
                if self.cursor_y < self.number_of_rows {
                    self.cursor_y += 1
                }
            }
            KeyCode::End => self.cursor_x = self.screen_size.0 - 1,
            KeyCode::Home => self.cursor_x = 0,
            _ => {
                unimplemented!()
            }
        }

        let current_row = if self.cursor_y < self.number_of_rows {
            Some(editor_rows.get_row(self.cursor_y))
        } else {
            None
        };
        self.cursor_x = cmp::min(
            self.cursor_x,
            match current_row {
                Some(row) => row.len(),
                _ => 0,
            },
        )
    }
}

struct EditorRows {
    row_content: Vec<String>,
    render: Vec<String>,
}

impl EditorRows {
    fn new() -> Self {
        let arg = std::env::args().collect::<Vec<String>>();
        match arg.get(1) {
            None => Self {
                row_content: Vec::new(),
                render: Vec::new(),
            },
            Some(file) => EditorRows::from_file(file.as_ref()),
        }
    }

    fn from_file(file: &Path) -> EditorRows {
        let file_contents = std::fs::read_to_string(file).expect("Unable to read file");
        //let render = Vec::new();

        let (render, row_content): (Vec<String>, Vec<String>) = file_contents
            .lines()
            .map(|it| {
                let capacity = it
                    .chars()
                    .fold(0, |acc, next| acc + if next == '\t' { 8 } else { 1 });
                let mut p = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open("check.txt")
                    .unwrap();

                let mut row = String::with_capacity(capacity);
                let mut index = 0;
                it.chars().for_each(|c| {
                    index += 1;
                    if c == '\t' {
                        row.push(' ');
                        p.write_all(format!("before {} row {}\n", index, row).as_bytes())
                            .unwrap();
                        while index % 8 != 0 {
                            row.push(' ');
                            index += 1
                        }
                        p.write_all(format!("after {} {}\n", index, row).as_bytes())
                            .unwrap();
                    } else {
                        row.push(c);
                    }
                });

                (row, it.into())
            })
            .unzip();

        let file: String = render.iter().map(|it| format!("{}\n", it)).collect();
        std::fs::write("render.txt", file).unwrap();
        Self {
            row_content,
            render,
        }
    }

    fn number_of_rows(&self) -> usize {
        self.row_content.len()
    }

    fn get_row(&self, at: usize) -> &str {
        &self.row_content[at]
    }

    fn get_render_row(&self, at: usize) -> &str {
        &self.render[at]
    }
}

struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    fn new() -> Self {
        Self {
            reader: Reader::new(),
            output: Output::new(),
        }
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.reader.key_pressed()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            } => return Ok(false),
            KeyEvent {
                code: val @ (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right),
                modifiers: KeyModifiers::NONE,
            } => self.output.move_cursor(val),
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
            } => (0..self.output.win_size.1).for_each(|_| {
                self.output.move_cursor(if matches!(val, KeyCode::PageUp) {
                    KeyCode::Up
                } else {
                    KeyCode::Down
                });
            }),
            KeyEvent {
                code: val @ (KeyCode::Home | KeyCode::End),
                modifiers: KeyModifiers::NONE,
            } => self.output.move_cursor(val),
            _ => {}
        }
        Ok(true)
    }

    fn run(&mut self) -> crossterm::Result<bool> {
        self.output.editor_refresh_screen()?;
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
