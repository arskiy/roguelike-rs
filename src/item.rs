use crate::curses::{Status, PLAYER};
use crate::game::{Game, PlayerAction};

const HEAL_AMOUNT: i32 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Heal,
}

enum UseResult {
    UsedUp,
    Cancelled,
}

pub fn use_item(inv_id: usize, game: &mut Game) -> PlayerAction {
    if let Some(item) = game.inventory[inv_id].item {
        let on_use = match item {
            Item::Heal => cast_heal,
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
    let mut player = &mut game.graphics.objects.borrow_mut()[PLAYER];
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
