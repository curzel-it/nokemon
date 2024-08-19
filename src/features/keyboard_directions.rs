use crate::{game_engine::{entity::Entity, keyboard_events_provider::KeyboardState}, maps::tiles::{entity_is_near_tile, entity_is_on_tile}};

pub fn set_direction_according_to_keyboard_state(entity: &mut dyn Entity, keyboard_state: &KeyboardState) {
    let new_direction = keyboard_state.direction_based_on_down_keys();

    println!("Hero frame {:#?}", entity.body().frame);

    if let Some(new_direction) = new_direction {
        if new_direction != entity.body().direction && entity_is_near_tile(entity) {
            entity.body_mut().current_speed = 0.0;
            entity.snap_to_nearest_tile();
            entity.body_mut().reset_speed();
            entity.body_mut().direction = new_direction;
        }
    } else {
        if entity_is_near_tile(entity) {
            entity.body_mut().current_speed = 0.0;
            entity.snap_to_nearest_tile();
        } else {
            // ...
        }    
    }
}