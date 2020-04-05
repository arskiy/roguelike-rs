use crate::curses::{Status, PLAYER};
use crate::game::Game;
use crate::object;
use crate::object::Object;

pub fn take_turn(monster_id: usize, game: &mut Game) {
    // only move if close
    if game.graphics.objects.borrow()[monster_id]
        .distance_to(&game.graphics.objects.borrow()[PLAYER])
        <= 10.0
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
            let monster = &game.graphics.objects.borrow()[monster_id];
            game.graphics
                .statuses
                .push(Status::new(format!("{} attacks you!", monster.name), 1));
        }
    }
}
