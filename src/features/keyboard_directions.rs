use crate::{constants::STEP_INPUT_THRESHOLD, game_engine::{concrete_entity::ConcreteEntity, entity::Entity}, utils::directions::Direction};

pub fn set_direction_based_on_current_keys(entity: &mut dyn Entity, new_direction: Direction) {
    let offset = entity.body().offset;
    let current_direction = entity.body().direction;

    if offset.x.abs() < STEP_INPUT_THRESHOLD && offset.y.abs() < STEP_INPUT_THRESHOLD {
        if new_direction != Direction::Unknown {
            entity.body_mut().reset_speed();
            entity.body_mut().direction = new_direction;
        } else {
            entity.body_mut().current_speed = 0.0;
        }
    } else {
        entity.body_mut().direction = current_direction;
    }
}

impl ConcreteEntity {
    pub fn set_direction_based_on_current_keys(&mut self, new_direction: Direction) {
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