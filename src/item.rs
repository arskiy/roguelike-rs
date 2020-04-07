use crate::curses::{Status, PLAYER};
use crate::game::{Game, PlayerAction};
use crate::object;
use crate::object::Object;

const HEAL_AMOUNT: i32 = 5;

const LIGHTNING_DAMAGE: i32 = 40;
const LIGHTNING_RANGE: i32 = 5;

const CONFUSION_RANGE: i32 = 8;
const CONFUSION_NUM_TURNS: i32 = 10;

const FIRE_RADIUS: i32 = 3;
const FIRE_DAMAGE: i32 = 12;
const FIRE_SELF_DAMAGE: i32 = 3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Heal,
    Lightning,
    Confusion,
    Fire,
    Sword,
    Shield,
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// An object that can be equipped, yielding bonuses.
pub struct Equipment {
    pub slot: Slot,
    pub equipped: bool,
    pub power_bonus: i32,
    pub max_hp_bonus: i32,
    pub defense_bonus: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Slot {
    LeftHand,
    RightHand,
    Head,
}

enum UseResult {
    UsedUp,
    UsedAndKept,
    Cancelled,
}

pub fn use_item(inv_id: usize, game: &mut Game) -> PlayerAction {
    if let Some(item) = game.inventory[inv_id].item {
        let on_use = match item {
            Item::Heal => cast_heal,
            Item::Lightning => cast_lightning,
            Item::Confusion => cast_confusion,
            Item::Fire => cast_fire,
            Item::Sword => toggle_equipment,
            Item::Shield => toggle_equipment,
        };
        match on_use(inv_id, game) {
            UseResult::UsedUp => {
                game.inventory.remove(inv_id);
            }
            UseResult::UsedAndKept => (),
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
        if fighter.hp == player.max_hp(&game.inventory) {
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

fn toggle_equipment(inv_id: usize, game: &mut Game) -> UseResult {
    let equipment = match game.inventory[inv_id].equipment {
        Some(equipment) => equipment,
        None => return UseResult::Cancelled,
    };
    if equipment.equipped {
        game.inventory[inv_id].dequip(&mut game.graphics.statuses);
    } else {
        game.inventory[inv_id].equip(&mut game.graphics.statuses);
    }

    if let Some(current) = object::get_equipped_in_slot(equipment.slot, &game.inventory) {
        game.inventory[current].dequip(&mut game.graphics.statuses);
    }

    UseResult::UsedAndKept
}

fn cast_lightning(_inv_id: usize, game: &mut Game) -> UseResult {
    let monster_id = closest_monster(&game.graphics.objects.borrow().clone(), LIGHTNING_RANGE);
    if let Some(monster_id) = monster_id {
        game.graphics.add_status(
            format!(
                "Zapt! A thunder strikes {} doing {} damage!",
                game.graphics.objects.borrow()[monster_id].name,
                LIGHTNING_DAMAGE
            ),
            1,
        );

        let objs = &mut game.graphics.objects.borrow_mut();
        if let Some(xp) =
            objs[monster_id].take_damage(LIGHTNING_DAMAGE, &mut game.graphics.statuses)
        {
            objs[PLAYER].fighter.as_mut().unwrap().xp += xp;
        };
        UseResult::UsedUp
    } else {
        game.graphics
            .add_status("No enemy is close enough.".to_string(), 1);
        UseResult::Cancelled
    }
}

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Slot::LeftHand => write!(f, "left hand"),
            Slot::RightHand => write!(f, "right hand"),
            Slot::Head => write!(f, "head"),
        }
    }
}

fn cast_confusion(_inv_id: usize, game: &mut Game) -> UseResult {
    use crate::ai::AI;

    let monster_id = closest_monster(&game.graphics.objects.borrow(), CONFUSION_RANGE);
    if let Some(monster_id) = monster_id {
        game.graphics.add_status(
            format!(
                "You confused {}!",
                game.graphics.objects.borrow()[monster_id].name,
            ),
            1,
        );
        let mut objs = game.graphics.objects.borrow_mut();
        let old_ai = objs[monster_id].ai.take().unwrap_or(AI::Basic);
        objs[monster_id].ai = Some(AI::Confused {
            prev_ai: Box::new(old_ai),
            turns: CONFUSION_NUM_TURNS,
        });

        UseResult::UsedUp
    } else {
        game.graphics
            .add_status("No enemy is close enough.".to_string(), 1);
        UseResult::Cancelled
    }
}

fn cast_fire(_inv_id: usize, game: &mut Game) -> UseResult {
    game.graphics.add_status(
        format!(
            "A wall of fire is created in the {} tiles around you!",
            FIRE_RADIUS
        ),
        1,
    );

    let player = game.graphics.objects.borrow()[PLAYER].clone();
    let mut xp_to_gain = 0;

    let mut objs = game.graphics.objects.borrow_mut();
    for obj in objs.iter_mut() {
        if obj.distance_to(&player) <= FIRE_RADIUS as f32 && obj.fighter.is_some() {
            if obj.name == "player" {
                game.graphics.statuses.push(Status::new(
                    format!("You caught fire for {} hp.", FIRE_SELF_DAMAGE),
                    1,
                ));
                obj.take_damage(FIRE_SELF_DAMAGE, &mut game.graphics.statuses);
            } else {
                game.graphics.statuses.push(Status::new(
                    format!(
                        "The {} gets burned for {} hit points.",
                        obj.name, FIRE_DAMAGE
                    ),
                    1,
                ));
                if let Some(xp) = obj.take_damage(FIRE_DAMAGE, &mut game.graphics.statuses) {
                    xp_to_gain += xp;
                }
            }
        }
    }

    objs[PLAYER].fighter.as_mut().unwrap().xp += xp_to_gain;

    UseResult::UsedUp
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
