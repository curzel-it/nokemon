use crate::game_engine::{entity::Entity, game::Game, game_state_update::GameStateUpdate};

pub fn handle_collisions_for_bullet(bullet: &dyn Entity, game: &Game) -> Vec<GameStateUpdate> {
    let damage = -bullet.dp();

    return game.collisions
        .get(&bullet.id())
        .unwrap_or(&vec![])
        .iter()
        .map(|victim_id| GameStateUpdate::IncreaseHp(*victim_id, damage))
        .collect();
}