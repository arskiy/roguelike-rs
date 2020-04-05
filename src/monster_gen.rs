use crate::map_gen::Rect;
use crate::object::{Fighter, Object, AI};

use rand::Rng;

const MAX_ROOM_MONSTERS: i32 = 3;

pub fn spawn(room: Rect, objects: &mut Vec<Object>) {
    let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

    for _ in 0..num_monsters {
        // choose random spot for this monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        let mut monster = if rand::random::<f32>() < 0.8 {
            let mut orc = Object::new(x, y, 'o', pancurses::COLOR_GREEN, false, "orc", true);
            orc.fighter = Some(Fighter {
                max_hp: 10,
                hp: 10,
                defence: 0,
                power: 3,
            });
            orc.ai = Some(AI::Basic);
            orc
        } else {
            let mut troll = Object::new(x, y, 'T', pancurses::COLOR_YELLOW, false, "troll", true);
            troll.fighter = Some(Fighter {
                max_hp: 16,
                hp: 16,
                defence: 1,
                power: 4,
            });
            troll.ai = Some(AI::Basic);
            troll
        };

        monster.alive = true;

        objects.push(monster);
    }
}
