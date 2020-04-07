use crate::ai::AI;
use crate::curses::Status;
use crate::game::Game;
use crate::item::{Equipment, Item, Slot};
use crate::tile::{is_blocked, Map};
use pancurses::A_BOLD;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub hp: i32,
    pub xp: i32,
    pub base_power: i32,
    pub base_defence: i32,
    pub base_max_hp: i32,
}

#[derive(Clone)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub ch: char,
    color: i16,
    is_bold: bool,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
    pub fighter: Option<Fighter>,
    pub ai: Option<AI>,
    pub item: Option<Item>,
    pub level: i32,
    pub level_up_xp: i32,
    pub equipment: Option<Equipment>,
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
            fighter: None,
            ai: None,
            item: None,
            level: 1,
            level_up_xp: 0,
            equipment: None,
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

    /// return the distance to another object
    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn take_damage(&mut self, damage: i32, statuses: &mut Vec<Status>) -> Option<i32> {
        // apply damage if possible
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }

            if fighter.hp < 0 {
                statuses.push(Status::new(format!("{} is dead!", self.name), 1));

                self.alive = false;
                self.ch = '%';
                self.color = pancurses::COLOR_RED;
                self.blocks = false;
                self.ai = None;
                self.name = format!("remains of {}", self.name);

                return Some(fighter.xp);
            }
        }

        None
    }

    pub fn attack(&mut self, target: &mut Object, statuses: &mut Vec<Status>, inv: &Vec<Object>) {
        let damage = self.power(inv) - target.defence(inv);
        if damage > 0 {
            statuses.push(Status::new(
                format!("{} attacks {} for {} hp.", self.name, target.name, damage),
                1,
            ));
            if let Some(xp) = target.take_damage(damage, statuses) {
                self.fighter.as_mut().unwrap().xp += xp;
            }
        } else {
            statuses.push(Status::new(
                format!("{} attacks {}, but has no effect.", self.name, target.name),
                1,
            ));
        }
    }

    pub fn heal(&mut self, amount: i32) {
        if let Some(ref mut fighter) = self.fighter {
            fighter.hp += amount;
            if fighter.hp > fighter.base_max_hp {
                fighter.hp = fighter.base_max_hp;
            }
        }
    }
    pub fn power(&self, inv: &Vec<Object>) -> i32 {
        let base_power = self.fighter.map_or(0, |f| f.base_power);
        let bonus: i32 = self
            .get_all_equipped(inv)
            .iter()
            .map(|e| e.power_bonus)
            .sum();
        base_power + bonus
    }

    pub fn get_all_equipped(&self, inv: &Vec<Object>) -> Vec<Equipment> {
        if self.name == "player" {
            inv.iter()
                .filter(|item| item.equipment.map_or(false, |e| e.equipped))
                .map(|item| item.equipment.unwrap())
                .collect()
        } else {
            vec![] // other objects have no equipment
        }
    }

    pub fn defence(&self, inv: &Vec<Object>) -> i32 {
        let base_defence = self.fighter.map_or(0, |f| f.base_defence);
        let bonus: i32 = self
            .get_all_equipped(inv)
            .iter()
            .map(|e| e.defense_bonus)
            .sum();
        base_defence + bonus
    }

    pub fn max_hp(&self, inv: &Vec<Object>) -> i32 {
        let base_max_hp = self.fighter.map_or(0, |f| f.base_max_hp);
        let bonus: i32 = self
            .get_all_equipped(inv)
            .iter()
            .map(|e| e.max_hp_bonus)
            .sum();
        base_max_hp + bonus
    }

    pub fn equip(&mut self, statuses: &mut Vec<Status>) {
        if self.item.is_none() {
            statuses.push(Status::new(
                format!("Can't equip {} because it's not an Item.", self.name),
                1,
            ));
            return;
        };
        if let Some(ref mut equipment) = self.equipment {
            if !equipment.equipped {
                equipment.equipped = true;
                statuses.push(Status::new(
                    format!("Equipped {} on {}.", self.name, equipment.slot),
                    1,
                ));
            }
        } else {
            statuses.push(Status::new(
                format!("Can't equip {} because it's not an Equipment.", self.name),
                1,
            ));
        }
    }

    pub fn dequip(&mut self, statuses: &mut Vec<Status>) {
        if self.item.is_none() {
            statuses.push(Status::new(
                format!("Can't dequip {} because it's not an Item.", self.name),
                1,
            ));
            return;
        };
        if let Some(ref mut equipment) = self.equipment {
            if equipment.equipped {
                equipment.equipped = false;
                statuses.push(Status::new(
                    format!("Dequipped {} from {}.", self.name, equipment.slot),
                    1,
                ));
            }
        } else {
            statuses.push(Status::new(
                format!("Can't dequip {} because it's not an Equipment.", self.name),
                1,
            ));
        }
    }
}

pub fn get_equipped_in_slot(slot: Slot, inventory: &[Object]) -> Option<usize> {
    for (inventory_id, item) in inventory.iter().enumerate() {
        if item
            .equipment
            .as_ref()
            .map_or(false, |e| e.equipped && e.slot == slot)
        {
            return Some(inventory_id);
        }
    }
    None
}

pub fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
    let (x, y) = objects[id].pos();
    if !is_blocked(x + dx, y + dy, map, objects) {
        objects[id].set_pos(x + dx, y + dy);
    }
}

/// will cause an object (monster, usually) to move towards a position (the player’s coordinates, usually).
pub fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
    // vector from this object to the target, and distance
    let dx = target_x - objects[id].x;
    let dy = target_y - objects[id].y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    // normalize it to length 1 (preserving direction), then round it and
    // convert to integer so the movement is restricted to the map grid
    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    move_by(id, dx, dy, map, objects);
}
