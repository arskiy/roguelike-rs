use crate::ai::AI;
use crate::item::Item;
use crate::map_gen::Rect;
use crate::object::{Fighter, Object};
use crate::tile::{is_blocked, Map};

use rand::distributions::{IndependentSample, Weighted, WeightedChoice};
use rand::Rng;

const MAX_ROOM_MONSTERS: i32 = 4;
const MAX_ROOM_ITEMS: i32 = 3;

pub fn spawn(room: Rect, objects: &mut Vec<Object>, map: &Map) {
    let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

    let monster_chances = &mut [
        Weighted {
            weight: 80,
            item: "orc",
        },
        Weighted {
            weight: 20,
            item: "troll",
        },
    ];

    let item_chances = &mut [
        Weighted {
            weight: 50,
            item: Item::Heal,
        },
        Weighted {
            weight: 20,
            item: Item::Fire,
        },
        Weighted {
            weight: 20,
            item: Item::Lightning,
        },
        Weighted {
            weight: 10,
            item: Item::Confusion,
        },
    ];

    for _ in 0..num_monsters {
        // choose random spot for this monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        if !is_blocked(x, y, map, objects) {
            let monster_choice = WeightedChoice::new(monster_chances);
            let mut monster = match monster_choice.ind_sample(&mut rand::thread_rng()) {
                "orc" => {
                    let mut orc =
                        Object::new(x, y, 'o', pancurses::COLOR_GREEN, false, "orc", true);
                    orc.fighter = Some(Fighter {
                        max_hp: 10,
                        hp: 10,
                        defence: 0,
                        xp: 35,
                        power: 3,
                    });
                    orc.ai = Some(AI::Basic);
                    orc
                }
                "troll" => {
                    let mut troll =
                        Object::new(x, y, 'T', pancurses::COLOR_YELLOW, false, "troll", true);
                    troll.fighter = Some(Fighter {
                        max_hp: 16,
                        hp: 16,
                        defence: 1,
                        xp: 100,
                        power: 4,
                    });
                    troll.ai = Some(AI::Basic);
                    troll
                }
                _ => unreachable!(),
            };

            monster.alive = true;

            objects.push(monster);
        }
    }

    let num_items = rand::thread_rng().gen_range(0, MAX_ROOM_ITEMS + 1);

    for _ in 0..num_items {
        // choose random spot for this item
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        // only place it if the tile is not blocked
        if !is_blocked(x, y, map, objects) {
            // create a healing potion
            let item_choice = WeightedChoice::new(item_chances);
            let item = match item_choice.ind_sample(&mut rand::thread_rng()) {
                Item::Heal => {
                    let mut object = Object::new(
                        x,
                        y,
                        '!',
                        pancurses::COLOR_MAGENTA,
                        false,
                        "healing potion",
                        false,
                    );
                    object.item = Some(Item::Heal);
                    object
                }
                Item::Lightning => {
                    let mut object = Object::new(
                        x,
                        y,
                        '#',
                        pancurses::COLOR_CYAN,
                        false,
                        "scroll of lightning",
                        false,
                    );
                    object.item = Some(Item::Lightning);
                    object
                }
                Item::Fire => {
                    let mut object = Object::new(
                        x,
                        y,
                        '#',
                        pancurses::COLOR_RED,
                        false,
                        "scroll of fire",
                        false,
                    );
                    object.item = Some(Item::Fire);
                    object
                }

                Item::Confusion => {
                    let mut object = Object::new(
                        x,
                        y,
                        '#',
                        pancurses::COLOR_GREEN,
                        false,
                        "scroll of confusion",
                        false,
                    );
                    object.item = Some(Item::Confusion);
                    object
                }
            };

            objects.push(item);
        }
    }
}
