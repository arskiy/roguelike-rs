use std::cell::RefCell;

use crate::object::Object;
use crate::tile::{Map, MAP_HEIGHT, MAP_WIDTH};
use pancurses::{Window, A_BOLD};

pub const SCR_WIDTH: i32 = 130;
pub const INV_X: i32 = 101;

pub const WINDOW_WIDTH: i32 = 99;
pub const WINDOW_HEIGHT: i32 = 40;

pub const STATUS_Y: i32 = 32;
pub const STATUS_HEIGHT: i32 = WINDOW_HEIGHT - STATUS_Y;

pub const PLAYER_STATS_X: i32 = WINDOW_WIDTH / 2 + 1;

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
    pub fn draw(&mut self, map: &Map) {
        self.window.clear();

        self.draw_borders();

        for obj in &*self.objects.borrow() {
            obj.draw(&self.window);
        }

        // draw alive objects with priority
        let _ = self
            .objects
            .borrow()
            .iter()
            .filter(|obj| obj.alive)
            .map(|obj| obj.draw(&self.window));

        // draw player with priority
        self.objects.borrow()[PLAYER].draw(&self.window);

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let wall = map[x as usize][y as usize].block_sight;
                if wall {
                    self.window.mvaddch(y, x, '+');
                }
            }
        }

        self.window.mvaddstr(STATUS_Y - 2, 1, "Message log:");

        for y in (STATUS_Y - 2)..WINDOW_HEIGHT {
            self.window.mvaddch(y, WINDOW_WIDTH / 2, '|');
        }

        for (i, status) in self.statuses.iter_mut().enumerate() {
            self.window
                .mvaddstr(STATUS_Y - 1 + i as i32, 1, status.msg.clone());
            status.rounds -= 1;
        }

        self.statuses = self
            .statuses
            .clone()
            .into_iter()
            .filter(|status| status.rounds != 0)
            .collect();

        self.window.refresh();
    }

    pub fn add_status(&mut self, msg: String, rounds: u32) {
        self.statuses.push(Status::new(msg, rounds));
    }

    pub fn draw_player_stats(&self, player: &mut Object) {
        if player.alive && player.fighter.unwrap().hp > 0 {
            let hp = player.fighter.unwrap().hp;

            if hp < 10 {
                self.window.color_set(pancurses::COLOR_RED);
            } else {
                self.window.color_set(pancurses::COLOR_GREEN);
            }

            self.window.mvaddstr(
                STATUS_Y - 2,
                PLAYER_STATS_X,
                format!(
                    "HP: {}/{}",
                    player.fighter.unwrap().hp,
                    player.fighter.unwrap().max_hp
                ),
            );

            self.window.color_set(pancurses::COLOR_YELLOW);
            self.window.mvaddstr(
                STATUS_Y - 1,
                PLAYER_STATS_X,
                format!("Defence: {}", player.fighter.unwrap().defence),
            );

            self.window.color_set(pancurses::COLOR_CYAN);
            self.window.mvaddstr(
                STATUS_Y,
                PLAYER_STATS_X,
                format!("Power: {}", player.fighter.unwrap().power),
            );

            self.window.color_set(pancurses::COLOR_WHITE);
        } else {
            player.alive = false;
            player.ch = '%';

            self.window.color_set(pancurses::COLOR_RED);
            self.window.attron(A_BOLD);
            self.window
                .mvaddstr(STATUS_Y - 2, PLAYER_STATS_X, "You're dead!");
            self.window.attroff(A_BOLD);
            self.window.color_set(pancurses::COLOR_WHITE);
        }
    }

    pub fn push_obj(&mut self, obj: Object) {
        self.objects.borrow_mut().push(obj);
    }

    fn draw_borders(&self) {
        for i in 0..SCR_WIDTH {
            self.window.mvaddch(0, i, '-');
            self.window.mvaddch(WINDOW_HEIGHT, i, '-');
        }

        for i in 0..WINDOW_HEIGHT {
            self.window.mvaddch(i, 0, '|');
            self.window.mvaddch(i, WINDOW_WIDTH, '|');
            self.window.mvaddch(i, SCR_WIDTH, '|');
        }

        self.window.mvaddch(0, 0, '+');
        self.window.mvaddch(WINDOW_HEIGHT, 0, '+');
        self.window.mvaddch(0, WINDOW_WIDTH, '+');
        self.window.mvaddch(0, SCR_WIDTH, '+');
        self.window.mvaddch(WINDOW_HEIGHT, WINDOW_WIDTH / 2, '+');
        self.window.mvaddch(WINDOW_HEIGHT, WINDOW_WIDTH, '+');
        self.window.mvaddch(WINDOW_HEIGHT, SCR_WIDTH, '+');
    }
}

impl Default for Graphics {
    fn default() -> Self {
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
}

impl Drop for Graphics {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
