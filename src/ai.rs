use crate::curses::{Status, PLAYER};
use crate::game::Game;
use crate::object;
use crate::object::Object;

pub fn take_turn(monster_id: usize, game: &mut Game) {
    let objects = game.graphics.objects.borrow();
    // only move if close
    if objects[monster_id].distance_to(&objects[PLAYER]) <= 10.0 {
        let (monster_x, monster_y) = objects[monster_id].pos();
        if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
            // move towards player if far away
            let (player_x, player_y) = objects[PLAYER].pos();
            object::move_towards(
                monster_id,
                player_x,
                player_y,
                &game.map,
                &mut game.graphics.objects.borrow_mut(),
            );
        } else if objects[PLAYER].fighter.map_or(false, |f| f.hp > 0) {
            // close enough, attack! (if the player is still alive.)
            let monster = &objects[monster_id];
            game.graphics.statuses.push(Status::new(
                format!(
                    "The attack of the {} bounces off your shiny metal armor!",
                    monster.name
                ),
                1,
            ));
        }
    }
}
