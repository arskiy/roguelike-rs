use std::cell::RefCell;

use crate::game;
use crate::game::{Game, PlayerAction};
use crate::object::Object;
use crate::tile::{Map, MAP_HEIGHT, MAP_WIDTH};
use pancurses::{Input, Window};

pub const WINDOW_WIDTH: i32 = 99;
pub const WINDOW_HEIGHT: i32 = 40;

pub const STATUS_Y: i32 = 30;
pub const STATUS_HEIGHT: i32 = 10;

pub const PLAYER: usize = 0;

/// Handles drawing. Expects player to be the first in the vector.
pub struct Graphics {
    pub objects: RefCell<Vec<Object>>,
    pub window: Window,
    pub statuses: Vec<Status>,
}

#[derive(Clone)]
pub struct Status {
    msg: String,
    rounds: u32,
}

impl Status {
    pub fn new(msg: String, rounds: u32) -> Self {
        Self { msg, rounds }
    }
}

impl Graphics {
    pub fn new() -> Self {
        let window = pancurses::initscr();

        window.keypad(true);
        pancurses::curs_set(0);
        pancurses::noecho();
        pancurses::start_color();

        pancurses::init_pair(1, pancurses::COLOR_RED, pancurses::COLOR_BLACK);
        pancurses::init_pair(2, pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
        pancurses::init_pair(3, pancurses::COLOR_YELLOW, pancurses::COLOR_BLACK);
        pancurses::init_pair(4, pancurses::COLOR_BLUE, pancurses::COLOR_BLACK);
        pancurses::init_pair(5, pancurses::COLOR_MAGENTA, pancurses::COLOR_BLACK);
        pancurses::init_pair(6, pancurses::COLOR_CYAN, pancurses::COLOR_BLACK);
        pancurses::init_pair(7, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);

        window.color_set(7);

        Self {
            objects: RefCell::new(Vec::new()),
            window,
            statuses: Vec::new(),
        }
    }

    pub fn draw(&mut self, map: &Map) {
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

        for (i, status) in self.statuses.iter_mut().enumerate() {
            self.window
                .mvaddstr(STATUS_Y + i as i32, 1, status.msg.clone());
            status.rounds -= 1;
        }

        self.statuses = self
            .statuses
            .clone()
            .into_iter()
            .filter(|status| if status.rounds != 0 { true } else { false })
            .collect();

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

    pub fn handle_keys(&self, game: &mut Game) -> PlayerAction {
        let is_alive = game.graphics.objects.borrow()[PLAYER].alive;
        match (self.window.getch(), is_alive) {
            (Some(Input::KeyDC), _) | (Some(Input::Character('q')), _) => {
                return PlayerAction::Exit
            } // exit game

            // movement keys
            (Some(Input::KeyUp), true) => {
                game::player_move_or_attack(0, -1, game, &mut game.graphics.objects.borrow_mut());
                PlayerAction::TookTurn
            }
            (Some(Input::KeyDown), true) => {
                game::player_move_or_attack(0, 1, game, &mut game.graphics.objects.borrow_mut());
                PlayerAction::TookTurn
            }
            (Some(Input::KeyLeft), true) => {
                game::player_move_or_attack(-1, 0, game, &mut game.graphics.objects.borrow_mut());
                PlayerAction::TookTurn
            }
            (Some(Input::KeyRight), true) => {
                game::player_move_or_attack(1, 0, game, &mut game.graphics.objects.borrow_mut());
                PlayerAction::TookTurn
            }

            (Some(_), _) | (None, _) => PlayerAction::DidntTakeTurn,
        }
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
