use crate::curses::{Graphics, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::object::Object;
use crate::tile;
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

        let mut player = Object::new(
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2,
            '@',
            pancurses::COLOR_WHITE,
            true,
            "player",
            true,
        );

        player.alive = true;

        graphics.push_obj(player);

        // procedurally generate the map
        self.map = tile::make_map(&mut graphics.objects.borrow_mut());

        loop {
            graphics.draw(&self.map);

            let exit = graphics.handle_keys(self, &mut graphics.objects.borrow_mut());
            if exit {
                break;
            }
        }
    }
}
