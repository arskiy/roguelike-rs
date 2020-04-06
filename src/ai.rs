use crate::curses::{Status, PLAYER};
use crate::game::Game;
use crate::object;
use crate::object::Object;
use std::cmp;

pub fn take_turn(monster_id: usize, game: &mut Game) {
    // only move if close
    if game.graphics.objects.borrow()[monster_id]
        .distance_to(&game.graphics.objects.borrow()[PLAYER])
        <= 6.0
    {
        // let (monster_x, monster_y) = game.graphics.objects.borrow()[monster_id].pos();
        if game.graphics.objects.borrow()[monster_id]
            .distance_to(&game.graphics.objects.borrow()[PLAYER])
            >= 2.0
        {
            // move towards player if far away
            let (player_x, player_y) = game.graphics.objects.borrow()[PLAYER].pos();
            object::move_towards(
                monster_id,
                player_x,
                player_y,
                &game.map,
                &mut game.graphics.objects.borrow_mut(),
            );
        } else if game.graphics.objects.borrow()[PLAYER]
            .fighter
            .map_or(false, |f| f.hp > 0)
        {
            // close enough, attack! (if the player is still alive.)
            /*
            let (mut monster, mut player) =
                mut_two(monster_id, PLAYER, &mut game.graphics.objects.borrow_mut());
            monster.attack(&mut player, &mut game.graphics);
            */
            game.graphics.objects.borrow()[monster_id].attack(
                &mut game.graphics.objects.borrow_mut()[PLAYER].clone(),
                &mut game.graphics,
            );
        }
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
