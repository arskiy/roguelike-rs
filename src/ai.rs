use crate::curses::{Status, PLAYER};
use crate::game::Game;
use crate::object;

use rand::Rng;
use std::cmp;

#[derive(Clone)]
pub enum AI {
    Basic,
    Confused { prev_ai: Box<AI>, turns: i32 },
}
pub fn take_turn(monster_id: usize, game: &mut Game) {
    // only move if close
    let monster = &game.graphics.objects.borrow()[monster_id].clone();
    let player = &game.graphics.objects.borrow()[PLAYER].clone();
    match &monster.ai {
        Some(AI::Basic) => {
            if monster.distance_to(&game.graphics.objects.borrow()[PLAYER]) <= 6.0 {
                if monster.distance_to(&game.graphics.objects.borrow()[PLAYER]) >= 2.0 {
                    // move towards player if far away
                    let (player_x, player_y) = player.pos();
                    object::move_towards(
                        monster_id,
                        player_x,
                        player_y,
                        &game.map,
                        &mut game.graphics.objects.borrow_mut(),
                    );
                } else if player.fighter.map_or(false, |f| f.hp > 0) {
                    // close enough, attack! (if the player is still alive.)
                    let mut objs = game.graphics.objects.borrow_mut();
                    let (monster, mut player) = mut_two(monster_id, PLAYER, &mut objs);
                    monster.attack(&mut player, &mut game.graphics.statuses, &game.inventory)
                }
            }
        }
        Some(AI::Confused { prev_ai, turns }) => {
            if turns >= &0 {
                // still confused ...
                // move in a random direction, and decrease the number of turns confused
                {
                    object::move_by(
                        monster_id,
                        rand::thread_rng().gen_range(-1, 2),
                        rand::thread_rng().gen_range(-1, 2),
                        &game.map,
                        &mut game.graphics.objects.borrow_mut(),
                    );
                }
                game.graphics.objects.borrow_mut()[monster_id].ai = Some(AI::Confused {
                    prev_ai: prev_ai.clone(),
                    turns: turns - 1,
                });
            } else {
                // restore the previous AI (this one will be deleted)
                game.graphics.statuses.push(Status::new(
                    format!("The {} is no longer confused!", monster.name),
                    1,
                ));
                game.graphics.objects.borrow_mut()[monster_id].ai = Some(*prev_ai.clone());
            }
        }
        _ => (),
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
