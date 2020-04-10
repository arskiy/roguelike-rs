use crate::ai;
use crate::curses::{Graphics, Status, INV_X, PLAYER, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::fov;
use crate::item;
use crate::item::{Equipment, Item, Slot};
use crate::object::{get_equipped_in_slot, move_by, Fighter, Object};
use crate::tile;
use crate::tile::{Map, Tile, MAP_HEIGHT, MAP_WIDTH};
use pancurses::Input;

const PLAYER_DEF_HP: i32 = 40;
const LEVEL_UP_BASE: i32 = 200;
const LEVEL_UP_FACTOR: i32 = 150;

pub struct Game {
    pub map: Map,
    pub graphics: Graphics,
    pub inventory: Vec<Object>,
    pub dungeon_level: u32,
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
            base_max_hp: PLAYER_DEF_HP,
            hp: PLAYER_DEF_HP,
            base_defence: 1,
            xp: 0,
            base_power: 4,
        });

        let mut dagger = Object::new(0, 0, '-', pancurses::COLOR_BLUE, false, "dagger", false);
        dagger.item = Some(Item::Sword);
        dagger.equipment = Some(Equipment {
            equipped: true,
            slot: Slot::LeftHand,
            max_hp_bonus: 0,
            defense_bonus: 0,
            power_bonus: 2,
        });
        self.inventory.push(dagger);

        player.level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;

        self.graphics.push_obj(player);

        // procedurally generate the map
        self.map = tile::make_map(&mut self.graphics.objects.borrow_mut(), self.dungeon_level);

        let mut points = vec![];
        for i in 0..MAP_WIDTH {
            points.push(fov::Point { x: i, y: 0 });
            points.push(fov::Point {
                x: i,
                y: MAP_HEIGHT,
            });
        }

        for i in 0..MAP_HEIGHT {
            points.push(fov::Point { x: 0, y: i });
            points.push(fov::Point { x: MAP_WIDTH, y: i });
        }

        loop {
            let names = self.get_names_under_player();
            if !names.is_empty() {
                self.graphics.add_status(names, 1)
            };

            /*
            {
                let player = &self.graphics.objects.borrow()[PLAYER];
                fov::raycast_on_map(
                    &mut self.map,
                    player.x,
                    player.y,
                    &mut self.graphics.statuses,
                );
            }
             */

            self.graphics.draw(&self.map);

            self.graphics.draw_player_stats(
                &mut self.graphics.objects.borrow_mut()[PLAYER],
                self.dungeon_level,
                &self.inventory,
            );

            self.show_inventory();

            self.level_up();

            let player_action = self.handle_keys();

            if let PlayerAction::Exit = player_action {
                break;
            }

            if self.graphics.objects.borrow()[PLAYER].alive
                && player_action != PlayerAction::DidntTakeTurn
            {
                let m = self.graphics.objects.borrow().len();
                for id in 0..m {
                    if self.graphics.objects.borrow()[id].ai.is_some() {
                        ai::take_turn(id, self);
                    }
                }
            }
        }
    }

    fn get_names_under_player(&self) -> String {
        let objs = self.graphics.objects.borrow();
        let (px, py) = objs[PLAYER].pos();

        let names = objs
            .iter()
            .filter(|obj| obj.pos() == (px, py) && obj.name != "player")
            .map(|obj| obj.name.clone())
            .collect::<Vec<_>>();

        names.join(", ")
    }

    pub fn handle_keys(&mut self) -> PlayerAction {
        let is_alive = self.graphics.objects.borrow()[PLAYER].alive;
        match (self.graphics.window.getch(), is_alive) {
            (Some(Input::KeyDC), _) | (Some(Input::Character('q')), _) => PlayerAction::Exit, // exit game

            (Some(Input::Character(',')), true) => {
                // let objs = self.graphics.objects.borrow();
                let item_id = self.graphics.objects.borrow().iter().position(|object| {
                    object.pos() == self.graphics.objects.borrow()[PLAYER].pos()
                        && object.item.is_some()
                });

                if let Some(item_id) = item_id {
                    self.pick_item_up(item_id);
                }
                PlayerAction::TookTurn
            }

            // rest, do nothing for a turn
            (Some(Input::Character('.')), true) => PlayerAction::TookTurn,

            // apply (use) an item
            (Some(Input::Character('a')), true) => self.apply_item(),

            (Some(Input::Character('d')), true) => self.drop_item(),

            // (Some(Input::Character('D')), true) => self.drop_item_by_type(),
            (Some(Input::Character('>')), true) => {
                let player_on_stairs = self.graphics.objects.borrow().iter().any(|object| {
                    object.pos() == self.graphics.objects.borrow()[PLAYER].pos()
                        && object.name == "stairs"
                });

                if player_on_stairs {
                    self.next_level();
                }

                PlayerAction::DidntTakeTurn
            }

            // movement keys
            (Some(Input::KeyUp), true) | (Some(Input::Character('k')), true) => {
                self.player_move_or_attack(0, -1);
                PlayerAction::TookTurn
            }
            (Some(Input::KeyDown), true) | (Some(Input::Character('j')), true) => {
                self.player_move_or_attack(0, 1);
                PlayerAction::TookTurn
            }
            (Some(Input::KeyLeft), true) | (Some(Input::Character('h')), true) => {
                self.player_move_or_attack(-1, 0);
                PlayerAction::TookTurn
            }
            (Some(Input::KeyRight), true) | (Some(Input::Character('l')), true) => {
                self.player_move_or_attack(1, 0);
                PlayerAction::TookTurn
            }

            (Some(_), _) | (None, _) => PlayerAction::DidntTakeTurn,
        }
    }

    /// add to the player's inventory and remove from the map
    pub fn pick_item_up(&mut self, object_id: usize) {
        if self.inventory.len() >= 26 {
            self.graphics.add_status(
                format!(
                    "Your inventory is full, cannot pick up {}.",
                    self.graphics.objects.borrow()[object_id].name
                ),
                1,
            );
        } else {
            let item = self.graphics.objects.borrow_mut().swap_remove(object_id);
            self.graphics
                .add_status(format!("You picked up a {}!", item.name), 1);
            let index = self.inventory.len();
            let slot = item.equipment.map(|e| e.slot);
            self.inventory.push(item);

            // automatically equip, if the corresponding equipment slot is unused
            if let Some(slot) = slot {
                if get_equipped_in_slot(slot, &self.inventory).is_none() {
                    self.inventory[index].equip(&mut self.graphics.statuses);
                }
            }
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
            .position(|object| object.pos() == (x, y) && object.alive);

        // attack if target found, move otherwise
        match target_id {
            Some(target_id) => {
                let mut objs = self.graphics.objects.borrow_mut();
                let (player, mut target) = ai::mut_two(PLAYER, target_id, &mut objs);

                player.attack(&mut target, &mut self.graphics.statuses, &self.inventory);
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

    fn next_level(&mut self) {
        self.graphics
            .add_status("You take a moment to rest.".to_string(), 1);
        let mut objs = self.graphics.objects.borrow_mut();
        let heal_hp = objs[PLAYER].max_hp(&self.inventory) / 2;
        objs[PLAYER].heal(heal_hp);
        self.dungeon_level += 1;
        self.map = tile::make_map(&mut objs, self.dungeon_level);
    }

    fn level_up(&mut self) {
        let level_up_xp =
            LEVEL_UP_BASE + self.graphics.objects.borrow()[PLAYER].level * LEVEL_UP_FACTOR;
        // see if the player's experience is enough to level-up
        if self.graphics.objects.borrow()[PLAYER].fighter.unwrap().xp >= level_up_xp {
            // it is! level up
            {
                let player = &mut self.graphics.objects.borrow_mut()[PLAYER];

                player.level += 1;
                player.level_up_xp = level_up_xp;
                self.graphics.statuses.push(Status::new(
                    format!(
                        "Your battle skills grow stronger! You reached level {}!",
                        player.level
                    ),
                    1,
                ));
            }

            let mut choice = None;

            while choice.is_none() {
                // keep asking until a choice is made
                self.graphics.statuses.push(Status::new(
                    "Level up! Choose a stat to raise: (press the respective number)".to_string(),
                    1,
                ));
                self.graphics
                    .statuses
                    .push(Status::new("0 - Constitution (+20 HP)".to_string(), 1));
                self.graphics
                    .statuses
                    .push(Status::new("1 - Strength (+1 power)".to_string(), 1));
                self.graphics
                    .statuses
                    .push(Status::new("2 - Agility (+1 defence)".to_string(), 1));

                self.graphics.draw(&self.map);
                self.graphics.draw_player_stats(
                    &mut self.graphics.objects.borrow_mut()[PLAYER],
                    self.dungeon_level,
                    &self.inventory,
                );

                let player = &mut self.graphics.objects.borrow_mut()[PLAYER];
                let fighter = player.fighter.as_mut().unwrap();
                choice = self.graphics.window.getch();
                match choice {
                    Some(Input::Character('0'..='2')) => {
                        fighter.xp -= level_up_xp;
                        match choice.unwrap() {
                            Input::Character('0') => {
                                fighter.base_max_hp += 20;
                                fighter.hp += 20;
                            }
                            Input::Character('1') => {
                                fighter.base_power += 1;
                            }
                            Input::Character('2') => {
                                fighter.base_defence += 1;
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => choice = None,
                }
            }
        }
    }

    // ------------------------------------
    // inventory-related methods
    fn show_inventory(&self) {
        self.graphics.window.color_set(pancurses::COLOR_WHITE);
        if self.inventory.len() > 0 {
            self.graphics.window.mvaddstr(1, INV_X, "Inventory:");
            for (i, item) in self.inventory.iter().enumerate() {
                if item.equipment.is_some() && item.equipment.unwrap().equipped {
                    self.graphics.window.mvaddstr(
                        (i + 3) as i32,
                        INV_X,
                        format!(
                            "{} - {} (on {})",
                            (i + 97) as u8 as char,
                            item.name.clone(),
                            item.equipment.unwrap().slot
                        ),
                    );
                } else {
                    self.graphics.window.mvaddstr(
                        (i + 3) as i32,
                        INV_X,
                        format!("{} - {}", (i + 97) as u8 as char, item.name.clone()),
                    );
                }
            }
        } else {
            self.graphics
                .window
                .mvaddstr(1, INV_X, "Your inventory is empty.");
        }
    }

    fn apply_item(&mut self) -> PlayerAction {
        self.graphics
            .add_status("PRESS A KEY TO USE AN ITEM:".to_string(), 1);
        self.graphics.draw(&self.map);
        self.show_inventory();
        self.graphics.draw_player_stats(
            &mut self.graphics.objects.borrow_mut()[PLAYER],
            self.dungeon_level,
            &self.inventory,
        );
        match self.graphics.window.getch() {
            Some(Input::Character(c)) => match c {
                'a'..='z' => {
                    let inv_id = c as u8 - 97;
                    if (inv_id as usize) < self.inventory.len() {
                        return item::use_item(inv_id as usize, self);
                    } else {
                        self.graphics
                            .add_status(format!("You don't have an item at {}.", c), 1);
                    }
                }
                _ => self
                    .graphics
                    .add_status("Please press a key from a to z.".to_string(), 1),
            },

            Some(Input::KeyDC) => self.graphics.add_status("Cancelled.".to_string(), 1),
            _ => self
                .graphics
                .add_status("Please press a key from a to z.".to_string(), 1),
        }
        PlayerAction::DidntTakeTurn
    }

    fn drop_item(&mut self) -> PlayerAction {
        self.graphics
            .add_status("PRESS A KEY TO DROP AN ITEM:".to_string(), 1);
        self.graphics.draw(&self.map);
        self.show_inventory();
        self.graphics.draw_player_stats(
            &mut self.graphics.objects.borrow_mut()[PLAYER],
            self.dungeon_level,
            &self.inventory,
        );
        match self.graphics.window.getch() {
            Some(Input::Character(c)) => match c {
                'a'..='z' => {
                    let inv_id = (c as u8 - 97) as usize;
                    if inv_id < self.inventory.len() {
                        if self.inventory[inv_id].equipment.is_some() {
                            self.inventory[inv_id].dequip(&mut self.graphics.statuses);
                        }
                        self.inventory.remove(inv_id);
                    } else {
                        self.graphics
                            .add_status(format!("You don't have an item at {}.", c), 1);
                    }
                }
                _ => self
                    .graphics
                    .add_status("Please press a key from a to z.".to_string(), 1),
            },

            Some(Input::KeyDC) => self.graphics.add_status("Cancelled.".to_string(), 1),
            _ => self
                .graphics
                .add_status("Please press a key from a to z.".to_string(), 1),
        }
        PlayerAction::DidntTakeTurn
    }

    fn drop_item_by_type(&mut self) -> PlayerAction {
        self.graphics
            .add_status("PRESS A KEY TO DROP AN ITEM BY TYPE:".to_string(), 1);
        self.graphics.draw(&self.map);
        self.show_inventory();
        self.graphics.draw_player_stats(
            &mut self.graphics.objects.borrow_mut()[PLAYER],
            self.dungeon_level,
            &self.inventory,
        );
        match self.graphics.window.getch() {
            Some(Input::Character(c)) => match c {
                'a'..='z' => {
                    let inv_id = (c as u8 - 97) as usize;
                    if inv_id < self.inventory.len() {
                        let item = self.inventory[inv_id].item;
                        for (i, it) in self.inventory.clone().iter().enumerate() {
                            if it.item == item {
                                self.inventory.remove(i);
                            }
                        }
                    } else {
                        self.graphics
                            .add_status(format!("You don't have an item at {}.", c), 1);
                    }
                }
                _ => self
                    .graphics
                    .add_status("Please press a key from a to z.".to_string(), 1),
            },

            Some(Input::KeyDC) => self.graphics.add_status("Cancelled.".to_string(), 1),
            _ => self
                .graphics
                .add_status("Please press a key from a to z.".to_string(), 1),
        }
        PlayerAction::DidntTakeTurn
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            map: vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize],
            graphics: Graphics::default(),
            inventory: vec![],
            dungeon_level: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}
