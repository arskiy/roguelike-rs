use crate::curses::{
    Graphics, Status, PLAYER, STATUS_HEIGHT, STATUS_Y, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use crate::object::{move_by, Object};
use crate::tile;
use crate::tile::{Map, Tile, MAP_HEIGHT, MAP_WIDTH};

pub struct Game {
    pub map: Map,
    pub graphics: Graphics,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize],
            graphics: Graphics::new(),
        }
    }

    pub fn start(&mut self) {
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

        self.graphics.push_obj(player);

        // procedurally generate the map
        self.map = tile::make_map(&mut self.graphics.objects.borrow_mut());

        loop {
            self.graphics.draw(&self.map);

            let player_action = self.graphics.handle_keys(self);

            if let PlayerAction::Exit = player_action {
                break;
            }

            if self.graphics.objects.borrow()[PLAYER].alive
                && player_action != PlayerAction::DidntTakeTurn
            {
                for object in &*self.graphics.objects.borrow() {
                    // only if object is not player
                    if (object as *const _) != (&self.graphics.objects.borrow()[PLAYER] as *const _)
                    {
                        self.graphics
                            .statuses
                            .push(Status::new(format!("The {} growls!", object.name), 1));
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

pub fn player_move_or_attack(dx: i32, dy: i32, game: &mut Game, objects: &mut Vec<Object>) {
    // the coordinates the player is moving to/attacking
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    // try to find an attackable object there
    let target_id = objects.iter().position(|object| object.pos() == (x, y));

    // attack if target found, move otherwise
    match target_id {
        Some(target_id) => {
            game.graphics.statuses.push(Status::new(
                format!(
                    "The {} laughs at your pathetic attempt!",
                    objects[target_id].name
                ),
                1,
            ));
        }
        None => {
            move_by(PLAYER, dx, dy, &game.map, objects);
        }
    }
}
