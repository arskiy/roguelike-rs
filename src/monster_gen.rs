use crate::map_gen::Rect;
use crate::object::Object;

use rand::Rng;

const MAX_ROOM_MONSTERS: i32 = 3;

pub fn spawn(room: Rect, objects: &mut Vec<Object>) {
    let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

    for _ in 0..num_monsters {
        // choose random spot for this monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        let mut monster = if rand::random::<f32>() < 0.8 {
            // 80% chance of getting an orc
            // create an orc
            Object::new(x, y, 'o', pancurses::COLOR_GREEN, false, "prc", true)
        } else {
            Object::new(x, y, 'T', pancurses::COLOR_YELLOW, false, "troll", true)
        };

        monster.alive = false;

        objects.push(monster);
    }
}
