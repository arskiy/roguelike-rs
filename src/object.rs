use crate::game::Game;

pub struct Object {
    x: i32,
    y: i32,
    ch: char,
    color: i16,
}

impl Object {
    pub fn new(x: i32, y: i32, ch: char, color: i16) -> Self {
        Self { x, y, ch, color }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, win: &pancurses::Window) {
        win.color_set(self.color);
        win.mvaddch(self.y, self.x, self.ch);
        // reset to white
        win.color_set(7);
    }
}
