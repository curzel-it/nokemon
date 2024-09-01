use crate::{constants::STEP_INPUT_THRESHOLD, game_engine::entity::Entity, utils::directions::Direction};

pub fn set_direction_according_to_keyboard(entity: &mut dyn Entity, new_direction: Direction) {
    let offset = entity.body().offset;
    let current_direction = entity.body().direction;

    if offset.x.abs() < STEP_INPUT_THRESHOLD && offset.y.abs() < STEP_INPUT_THRESHOLD {
        entity.body_mut().reset_speed();
        entity.body_mut().direction = new_direction;
    } else {
        entity.body_mut().direction = current_direction;
    }
}