use crate::curses::{Status, PLAYER};
use crate::game::{Game, PlayerAction};
use crate::object::Object;

const HEAL_AMOUNT: i32 = 4;

const LIGHTNING_DAMAGE: i32 = 40;
const LIGHTNING_RANGE: i32 = 5;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Heal,
    Lightning,
}

enum UseResult {
    UsedUp,
    Cancelled,
}

pub fn use_item(inv_id: usize, game: &mut Game) -> PlayerAction {
    if let Some(item) = game.inventory[inv_id].item {
        let on_use = match item {
            Item::Heal => cast_heal,
            Item::Lightning => cast_lightning,
        };
        match on_use(inv_id, game) {
            UseResult::UsedUp => {
                game.inventory.remove(inv_id);
            }
            UseResult::Cancelled => game
                .graphics
                .add_status("Cancelled item use.".to_string(), 1),
        }
        PlayerAction::TookTurn
    } else {
        game.graphics.add_status(
            format!("The {} cannot be used.", game.inventory[inv_id].name),
            1,
        );
        PlayerAction::DidntTakeTurn
    }
}
fn cast_heal(_inv_id: usize, game: &mut Game) -> UseResult {
    let player = &mut game.graphics.objects.borrow_mut()[PLAYER];
    if let Some(fighter) = player.fighter.as_mut() {
        if fighter.hp == fighter.max_hp {
            game.graphics.statuses.push(Status::new(
                "You are already at full health.".to_string(),
                1,
            ));
            return UseResult::Cancelled;
        }
        game.graphics.statuses.push(Status::new(
            "Your wounds start to feel better!".to_string(),
            1,
        ));
        player.heal(HEAL_AMOUNT);
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}

fn cast_lightning(_inv_id: usize, game: &mut Game) -> UseResult {
    let monster_id = closest_monster(&game.graphics.objects.borrow(), LIGHTNING_RANGE);
    if let Some(monster_id) = monster_id {
        game.graphics.add_status(
            format!(
                "Zapt! A thunder strikes {} doing {} damage!",
                game.graphics.objects.borrow()[monster_id].name,
                LIGHTNING_DAMAGE
            ),
            1,
        );
        game.graphics.objects.borrow_mut()[monster_id]
            .take_damage(LIGHTNING_DAMAGE, &mut game.graphics.statuses);
        UseResult::UsedUp
    } else {
        game.graphics
            .add_status("No enemy is close enough.".to_string(), 1);
        UseResult::Cancelled
    }
}

/// find closest enemy, up to a maximum range
fn closest_monster(objects: &[Object], max_range: i32) -> Option<usize> {
    let mut closest_enemy = None;
    let mut closest_dist = (max_range + 1) as f32; // start with (slightly more than) maximum range

    for (id, object) in objects.iter().enumerate() {
        if (id != PLAYER) && object.fighter.is_some() && object.ai.is_some() {
            // calculate distance between this object and the player
            let dist = objects[PLAYER].distance_to(object);
            if dist < closest_dist {
                // it's closer, so remember it
                closest_enemy = Some(id);
                closest_dist = dist;
            }
        }
    }
    closest_enemy
}
