use crate::curses::{Status, PLAYER};
use crate::game::Game;
use crate::object;
use crate::object::Object;
use crate::tile::Map;

use rand::Rng;
use std::cmp;

#[derive(Clone)]
pub enum AI {
    Basic,
    Confused { prev_ai: Box<AI>, turns: i32 },
}

pub fn take_turn(monster_id: usize, game: &mut Game) {
    let mut objs = game.graphics.objects.borrow_mut();
    let mut monster = &mut objs[monster_id];
    if let Some(ai) = monster.ai.take() {
        let new_ai = match ai {
            AI::Basic => basic_ai(
                monster_id,
                &mut objs,
                &game.map,
                &mut game.graphics.statuses,
            ),
            AI::Confused { prev_ai, turns } => confused_ai(
                monster_id,
                &mut objs,
                &game.map,
                &mut game.graphics.statuses,
                prev_ai,
                turns,
            ),
        };
        monster.ai = Some(new_ai);
    }
}

fn basic_ai(monster_id: usize, objs: &mut [Object], map: &Map, statuses: &mut Vec<Status>) -> AI {
    statuses.push(Status::new("entered basic ai".to_string(), 1));
    // only move if close
    if objs[monster_id].distance_to(&objs[PLAYER]) <= 6.0 {
        if objs[monster_id].distance_to(&objs[PLAYER]) >= 2.0 {
            // move towards player if far away
            let (player_x, player_y) = objs[PLAYER].pos();
            object::move_towards(monster_id, player_x, player_y, map, objs);
        } else if objs[PLAYER].fighter.map_or(false, |f| f.hp > 0) {
            // close enough, attack! (if the player is still alive.)
            let (monster, mut player) = mut_two(monster_id, PLAYER, objs);
            monster.attack(&mut player, statuses)
        }
    }
    AI::Basic
}

fn confused_ai(
    monster_id: usize,
    objs: &mut [Object],
    map: &Map,
    statuses: &mut Vec<Status>,
    prev_ai: Box<AI>,
    turns: i32,
) -> AI {
    if turns >= 0 {
        object::move_by(
            monster_id,
            rand::thread_rng().gen_range(-1, 2),
            rand::thread_rng().gen_range(-1, 2),
            map,
            objs,
        );

        AI::Confused {
            prev_ai,
            turns: turns - 1,
        }
    } else {
        statuses.push(Status::new(
            format!("The {} is no longer confused!", objs[monster_id].name),
            1,
        ));
        *prev_ai
    }
}

pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_index != second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}
