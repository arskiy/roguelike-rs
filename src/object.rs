use crate::curses::Status;
use crate::item::Item;
use crate::tile::{is_blocked, Map};
use pancurses::A_BOLD;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defence: i32,
    pub power: i32,
}

#[derive(Clone)]
pub enum AI {
    Basic,
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

    pub fn take_damage(&mut self, damage: i32, statuses: &mut Vec<Status>) {
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
                // self.fighter = None;
                self.ai = None;
                self.name = format!("remains of {}", self.name);
                // monster_death(self, statuses);
            }
        }
    }

    pub fn attack(&self, target: &mut Object, statuses: &mut Vec<Status>) {
        // a simple formula for attack damage
        let damage = self.fighter.map_or(0, |f| f.power) - target.fighter.map_or(0, |f| f.defence);
        if damage > 0 {
            // make the target take some damage
            statuses.push(Status::new(
                format!("{} attacks {} for {} hp.", self.name, target.name, damage),
                1,
            ));
            target.take_damage(damage, statuses);
            target.fighter.as_mut().unwrap().hp -= damage;
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
            if fighter.hp > fighter.max_hp {
                fighter.hp = fighter.max_hp;
            }
        }
    }
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
