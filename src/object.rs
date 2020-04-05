use crate::game::Game;
use crate::tile::is_blocked;
use pancurses::A_BOLD;

pub struct Object {
    pub x: i32,
    pub y: i32,
    ch: char,
    color: i16,
    is_bold: bool,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
}

impl Object {
    pub fn new(
        x: i32,
        y: i32,
        ch: char,
        color: i16,
        is_bold: bool,
        name: &str,
        blocks: bool,
    ) -> Self {
        Self {
            x,
            y,
            ch,
            color,
            is_bold,
            blocks,
            alive: false,
            name: name.into(),
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game, objects: &Vec<Object>) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked
            && !is_blocked(self.x + dx, self.y + dy, &game.map, objects)
        {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, win: &pancurses::Window) {
        win.color_set(self.color);
        if self.is_bold {
            win.attron(A_BOLD);
        }

        win.mvaddch(self.y, self.x, self.ch);

        if self.is_bold {
            win.attroff(A_BOLD);
        }
        win.color_set(pancurses::COLOR_WHITE);
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
