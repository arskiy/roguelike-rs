use crate::ai;
use crate::curses::{Graphics, Status, PLAYER, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::object::{move_by, Fighter, Object};
use crate::tile;
use crate::tile::{Map, Tile, MAP_HEIGHT, MAP_WIDTH};
use pancurses::Input;

pub struct Game {
    pub map: Map,
    pub graphics: Graphics,
}

impl Game {
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

        player.fighter = Some(Fighter {
            max_hp: 30,
            hp: 30,
            defence: 2,
            power: 5,
        });

        self.graphics.push_obj(player);

        // procedurally generate the map
        self.map = tile::make_map(&mut self.graphics.objects.borrow_mut());

        loop {
            self.graphics.draw(&self.map);
            self.graphics
                .draw_player_stats(&self.graphics.objects.borrow()[PLAYER]);

            let player_action = self.handle_keys();

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
                        /*
                        self.graphics
                            .statuses
                            .push(Status::new(format!("The {} growls!", object.name), 1));
                        */
                    }
                }

                let m = self.graphics.objects.borrow().len();
                for id in 0..m {
                    if self.graphics.objects.borrow()[id].ai.is_some() {
                        ai::take_turn(id, self);
                    }
                }
            }
        }
    }

    pub fn handle_keys(&mut self) -> PlayerAction {
        let is_alive = self.graphics.objects.borrow()[PLAYER].alive;
        match (self.graphics.window.getch(), is_alive) {
            (Some(Input::KeyDC), _) | (Some(Input::Character('q')), _) => PlayerAction::Exit, // exit game

            // movement keys
            (Some(Input::KeyUp), true) => {
                self.player_move_or_attack(0, -1);
                PlayerAction::TookTurn
            }
            (Some(Input::KeyDown), true) => {
                self.player_move_or_attack(0, 1);
                PlayerAction::TookTurn
            }
            (Some(Input::KeyLeft), true) => {
                self.player_move_or_attack(-1, 0);
                PlayerAction::TookTurn
            }
            (Some(Input::KeyRight), true) => {
                self.player_move_or_attack(1, 0);
                PlayerAction::TookTurn
            }

            (Some(_), _) | (None, _) => PlayerAction::DidntTakeTurn,
        }
    }

    pub fn player_move_or_attack(&mut self, dx: i32, dy: i32) {
        // the coordinates the player is moving to/attacking
        let x = self.graphics.objects.borrow()[PLAYER].x + dx;
        let y = self.graphics.objects.borrow()[PLAYER].y + dy;

        // try to find an attackable object there
        let target_id = self
            .graphics
            .objects
            .borrow_mut()
            .iter()
            .position(|object| object.pos() == (x, y));

        // attack if target found, move otherwise
        match target_id {
            Some(target_id) => {
                let (mut player, mut target) =
                    ai::mut_two(PLAYER, target_id, &mut self.graphics.objects.borrow_mut());
                player.attack(&mut target, &mut self.graphics);
            }
            None => {
                move_by(
                    PLAYER,
                    dx,
                    dy,
                    &self.map,
                    &mut self.graphics.objects.borrow_mut(),
                );
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            map: vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize],
            graphics: Graphics::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}
