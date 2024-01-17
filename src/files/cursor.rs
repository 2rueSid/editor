use std::io::{self, Write};
use termion::raw::RawTerminal;
use termion::scroll;

pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

impl Cursor {
    pub fn move_up(&mut self, stdout: &mut RawTerminal<io::Stdout>) {
        if self.y > 1 {
            self.y -= 1;
        }

        write!(stdout, "{}", termion::cursor::Goto(self.x, self.y)).unwrap();
    }

    pub fn move_down(&mut self, stdout: &mut RawTerminal<io::Stdout>) {
        self.y += 1;
        write!(stdout, "{}", termion::cursor::Goto(self.x, self.y)).unwrap();
    }

    pub fn move_right(&mut self, stdout: &mut RawTerminal<io::Stdout>) {
        self.x += 1;

        write!(stdout, "{}", termion::cursor::Goto(self.x, self.y)).unwrap();
    }

    pub fn move_left(&mut self, stdout: &mut RawTerminal<io::Stdout>) {
        if self.x > 1 {
            self.x -= 1;
        }

        write!(stdout, "{}", termion::cursor::Goto(self.x, self.y)).unwrap();
    }
}
