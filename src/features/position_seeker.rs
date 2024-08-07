use raylib::math::Vector2;

use crate::{game_engine::entity::Entity, utils::vector_utils::dumb_direction_vector};

pub fn set_direction_towards(entity: &mut dyn Entity, destination: &Vector2) {
    let current_position = entity.frame();
    let direction = dumb_direction_vector(
        current_position.x, 
        current_position.y, 
        destination.x,
        destination.y
    );
    entity.set_direction(direction);
}