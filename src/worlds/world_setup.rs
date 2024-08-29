use crate::{entities::hero::Hero, game_engine::{entity_body::EmbodiedEntity, world::World}, utils::vector::Vector2d};

impl World {
    pub fn setup(&mut self, source: &u32, hero_direction: &Vector2d) {
        let mut entity = Hero::new();
        if let Some(teleporter_position) = self.find_teleporter_for_destination(source) {
            let offset_x = if hero_direction.x > 0.0 { 1 } else { 0 };
            let offset_y = if hero_direction.y > 0.0 { 2 } else { 1 };
            entity.body_mut().frame.x = teleporter_position.x + hero_direction.x as u32 - offset_x;
            entity.body_mut().frame.y = teleporter_position.y + hero_direction.y as u32 - offset_y;
        } else {
            entity.center_in(&self.bounds);
        }
        entity.body_mut().direction = *hero_direction;
        entity.immobilize_for_seconds(0.2);
        
        if hero_direction.y < 0.0 {
            entity.body_mut().frame.y -= 1;            
        }
        if hero_direction.x > 0.0 {
            entity.body_mut().frame.x += 1;
        }
        if hero_direction.y > 0.0 {
            entity.body_mut().frame.y += 1;            
        }
        if hero_direction.x < 0.0 {
            entity.body_mut().frame.x -= 1;
        }
        self.add_entity(Box::new(entity));
    }    
}

