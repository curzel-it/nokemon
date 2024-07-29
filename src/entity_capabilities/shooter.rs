use raylib::math::Vector2;

use crate::{entities::{entity::EntityStateSnapshot, entity_capability::{EntityCapability, EntityStateUpdate, GameStateSnapshot}, factory::EntityDescriptor}, features::entity_locator::EntityLocator};

#[derive(Debug)]
pub struct Shooter {
    time_between_shots: f32,
    time_to_next_shot: f32,
}

impl Shooter {
    pub fn new(rpm: f32) -> Self {
        Self {
            time_between_shots: 1.0 / (rpm / 60.0),
            time_to_next_shot: 0.0,
        }
    }
}

impl EntityCapability for Shooter {
    fn update(&mut self, entity: &EntityStateSnapshot, _: &GameStateSnapshot, time_since_last_update: f32) -> EntityStateUpdate {
        self.time_to_next_shot -= time_since_last_update;

        if self.time_to_next_shot <= 0.0 {
            self.time_to_next_shot = self.time_between_shots - self.time_to_next_shot;
            let bullet = self.build_bullet(entity.frame.x, entity.frame.y);
            println!("Shooting! {:#?}", self.time_to_next_shot);
            return EntityStateUpdate::new_entity(bullet);
        } else {
            return EntityStateUpdate::nothing();
        }
    }
}

impl Shooter {
    fn build_bullet(&self, x: f32, y: f32) -> EntityDescriptor {
        EntityDescriptor {
            species: "towerdart".to_owned(),
            direction: Vector2::new(1.0, 0.0),
            origin: Vector2::new(x, y)
        }
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game::game::Game};

    #[test]
    fn can_move_on_update() {
        // ...
    }
}