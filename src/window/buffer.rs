use crate::motion::Motions;
use crate::window::cursor::Cursor;
use crate::window::piece_table::PieceTable;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, Read, Write};
use termion::clear;
use termion::raw::RawTerminal;

use termion::terminal_size;

#[derive(Clone)]
pub struct SegmentNode {
    pub value: String,
    pub line_number: usize,
}

pub struct Buffer {
    file_path: std::path::PathBuf,
    pub data: PieceTable,
    pub lines: usize,
    pub cursor: Cursor,
    pub segment: VecDeque<SegmentNode>,
    terminal_size: (u16, u16),
}

impl Buffer {
    pub fn new(path: Option<String>) -> Buffer {
        let file_path = match path {
            Some(v) => std::path::PathBuf::from(v),
            None => "".into(),
        };

        let file = match file_content(&file_path) {
            Ok(res) => res,
            Err(_) => String::new(),
        };

        let terminal_size = terminal_size().unwrap();
        let piece_table = PieceTable::new(&file);
        let initial_segment = piece_table.get_lines(0, terminal_size.1.into());

        Buffer {
            file_path,
            data: piece_table,
            lines: file.lines().count().clone(),
            cursor: Cursor { x: 1, y: 1 },
            terminal_size,
            segment: initial_segment,
        }
    }

    pub fn display_segment(&self, stdout: &mut RawTerminal<io::Stdout>) {
        let mut lines = String::new();

        for line in self.segment.iter() {
            lines.push_str(&line.value);
        }
        stdout.suspend_raw_mode().unwrap();
        write!(stdout, "{}{}", clear::All, lines,).unwrap();
        stdout.flush().unwrap();
        stdout.activate_raw_mode().unwrap();
    }

    pub fn motion(&mut self, motion: Motions, stdout: &mut RawTerminal<io::Stdout>) {
        match motion {
            Motions::Down => {
                if self.cursor.y + 1 >= self.terminal_size.1 {
                    self.data.next_line(&mut self.segment);
                    self.display_segment(stdout);
                } else {
                    self.cursor.move_down(self.terminal_size.1);
                }
            }
            Motions::Up => {
                if usize::from(self.cursor.y) > 1 {
                    self.cursor.move_up();
                }

                let ln = self.segment.front().unwrap().line_number;
                if usize::from(self.cursor.y) == ln {
                    self.data.prev_line(&mut self.segment);
                    self.display_segment(stdout);
                }
            }
            Motions::Left => self.cursor.move_left(),
            Motions::Right => self.cursor.move_right(),
        }

        write!(
            stdout,
            "{}",
            termion::cursor::Goto(self.cursor.x, self.cursor.y)
        )
        .unwrap();
        stdout.flush().unwrap();
    }
}

fn file_content(path: &std::path::PathBuf) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;
    Ok(content)
}