use std::cell::RefCell;

use crate::game::Game;
use crate::object::Object;
use crate::tile::{Map, Tile, MAP_HEIGHT, MAP_WIDTH};
use pancurses::{Input, Window};

pub const WINDOW_WIDTH: i32 = 100;
pub const WINDOW_HEIGHT: i32 = 40;

/// Handles drawing. Expects player to be the first in the vector.
pub struct Graphics {
    pub objects: RefCell<Vec<Object>>,
    pub window: Window,
}

impl Graphics {
    pub fn new() -> Self {
        let window = pancurses::initscr();

        window.keypad(true);
        pancurses::curs_set(0);
        pancurses::noecho();

        Self {
            objects: RefCell::new(Vec::new()),
            window,
        }
    }

    pub fn draw(&self, map: &Map) {
        self.window.clear();

        self.draw_borders();

        for obj in &*self.objects.borrow() {
            obj.draw(&self.window);
        }

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let wall = map[x as usize][y as usize].block_sight;
                if wall {
                    self.window.mvaddch(y, x, '+');
                }
            }
        }

        self.window.refresh();
    }

    pub fn push_obj(&mut self, obj: Object) {
        self.objects.borrow_mut().push(obj);
    }

    fn draw_borders(&self) {
        for i in 0..WINDOW_WIDTH {
            self.window.mvaddch(0, i, '-');
            self.window.mvaddch(WINDOW_HEIGHT, i, '-');
        }

        for i in 0..WINDOW_HEIGHT {
            self.window.mvaddch(i, 0, '|');
            self.window.mvaddch(i, WINDOW_WIDTH, '|');
        }

        self.window.mvaddch(0, 0, '+');
        self.window.mvaddch(WINDOW_HEIGHT, 0, '+');
        self.window.mvaddch(0, WINDOW_WIDTH, '+');
        self.window.mvaddch(WINDOW_HEIGHT, WINDOW_WIDTH, '+');
    }

    pub fn handle_keys(&self, player: &mut Object, game: &Game) -> bool {
        match self.window.getch() {
            Some(Input::KeyDC) | Some(Input::Character('q')) => return true, // exit game

            // movement keys
            Some(Input::KeyUp) => player.move_by(0, -1, game),
            Some(Input::KeyDown) => player.move_by(0, 1, game),
            Some(Input::KeyLeft) => player.move_by(-1, 0, game),
            Some(Input::KeyRight) => player.move_by(1, 0, game),

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
