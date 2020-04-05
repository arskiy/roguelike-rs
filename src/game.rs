use crate::curses::{Graphics, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::object::Object;
use crate::tile::{Map, Tile, MAP_HEIGHT, MAP_WIDTH};

pub struct Game {
    pub map: Map,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize],
        }
    }

    pub fn start(&mut self) {
        let mut graphics = Graphics::new();

        let player = Object::new(
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2,
            '@',
            pancurses::COLOR_WHITE,
        );
        graphics.push_obj(player);

        let npc = Object::new(
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2 - 5,
            '@',
            pancurses::COLOR_YELLOW,
        );
        graphics.push_obj(npc);

        self.map[30][22] = Tile::wall();
        self.map[50][22] = Tile::wall();

        loop {
            graphics.draw(&self.map);

            let exit = graphics.handle_keys(&mut graphics.objects.borrow_mut()[0], self);
            if exit {
                break;
            }
        }
    }
}
