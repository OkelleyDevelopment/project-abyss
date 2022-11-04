//! Needed structures to build the game window

extern crate pancurses;

use pancurses::{curs_set, endwin, initscr, Input, Window};

/// The Game window
///
/// This provides a wrapper around the pancurses window and gives room to expand
/// or shift implementations later on.
pub struct AbyssWindow {
    window: Window,
    buffer: String,
}

impl AbyssWindow {
    /// Creates the struct and sets some defaults
    pub fn new() -> Self {
        pancurses::cbreak();
        pancurses::noecho();
        curs_set(1);
        let w = AbyssWindow {
            window: initscr(),
            buffer: String::new(),
        };
        w.window.nodelay(true);
        w.window.keypad(false);
        w
    }

    pub fn clear(&self) {
        self.window.clear();
    }

    pub fn erase(&self) {
        self.window.erase();
    }

    pub fn print(&self, msg: &str) {
        self.window.printw(msg);
    }

    pub fn refresh(&self) {
        self.window.refresh();
    }

    pub fn endwindow(&self) {
        endwin();
        std::process::exit(0);
    }

    pub fn getch(&mut self) {
        match &self.window.getch() {
            Some(Input::Character(q)) if *q == 'q' || *q == 'Q' => {
                self.buffer.clear();
                self.window.clear();
                curs_set(0);
                self.endwindow();
            }
            Some(Input::Character(c)) => {
                if *c as u8 == 127 {
                    self.window.delch();
                    self.buffer.pop();
                } else {
                    self.buffer.push(*c);
                }
            }
            _ => {}
        }
    }

    pub fn display(&self) {
        self.print(self.buffer.as_str());
    }
}
