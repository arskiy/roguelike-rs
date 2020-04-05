use std::cell::RefCell;

use crate::object::Object;
use crate::pancurses::{Input, Window};

/// Handles drawing. Expects player to be the first in the vector.
pub struct Graphics {
    pub objects: RefCell<Vec<Object>>,
    pub window: Window,
}

impl Graphics {
    pub fn new() -> Self {
        let window = pancurses::initscr();
        let window_x = window.get_max_x() - 1;
        let window_y = window.get_max_y() - 1;

        window.keypad(true);
        pancurses::curs_set(0);
        pancurses::noecho();

        Self {
            objects: RefCell::new(Vec::new()),
            window,
        }
    }

    pub fn draw(&self) {
        self.window.clear();

        self.draw_borders();

        for obj in &*self.objects.borrow() {
            obj.draw(&self.window);
        }

        self.window.refresh();
    }

    pub fn push_obj(&mut self, obj: Object) {
        self.objects.borrow_mut().push(obj);
    }

    fn draw_borders(&self) {
        let max_x = self.window.get_max_x() - 1;
        let max_y = self.window.get_max_y() - 1;

        for i in 0..max_x {
            self.window.mvaddch(0, i, '-');
            self.window.mvaddch(max_y, i, '-');
        }

        for i in 0..max_y {
            self.window.mvaddch(i, 0, '|');
            self.window.mvaddch(i, max_x, '|');
        }

        self.window.mvaddch(0, 0, '+');
        self.window.mvaddch(max_y, 0, '+');
        self.window.mvaddch(0, max_x, '+');
        self.window.mvaddch(max_y, max_x, '+');
    }

    pub fn handle_keys(&self, player: &mut Object) -> bool {
        match self.window.getch() {
            Some(Input::KeyDC) | Some(Input::Character('q')) => return true, // exit game

            // movement keys
            Some(Input::KeyUp) => player.move_by(0, -1),
            Some(Input::KeyDown) => player.move_by(0, 1),
            Some(Input::KeyLeft) => player.move_by(-1, 0),
            Some(Input::KeyRight) => player.move_by(1, 0),

            Some(_) | None => (),
        }

        false
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
