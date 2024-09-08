use crate::{constants::STEP_INPUT_THRESHOLD, game_engine::concrete_entity::ConcreteEntity, utils::directions::Direction};

impl ConcreteEntity {
    pub fn update_direction_for_current_keys(&mut self, new_direction: Direction) {
        let current_direction = self.direction;

        if self.offset.x.abs() < STEP_INPUT_THRESHOLD && self.offset.y.abs() < STEP_INPUT_THRESHOLD {
            if new_direction != Direction::Unknown {
                self.reset_speed();
                self.direction = new_direction;
            } else {
                self.current_speed = 0.0;
            }
        } else {
            self.direction = current_direction;
        }
    }
}