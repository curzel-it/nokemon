use crate::{entities::hero::Hero, game_engine::{entity_body::EmbodiedEntity, world::World}, utils::directions::Direction};

impl World {
    pub fn setup(&mut self, source: &u32, hero_direction: &Direction) {
        self.compute_hitmap();

        let mut entity = Hero::new();
        if let Some(teleporter_position) = self.find_teleporter_for_destination(source) {
            let (offset_x, offset_y): (i32, i32) = match hero_direction {
                Direction::Up => (0, -2),
                Direction::Right => (1, -1),
                Direction::Down => (0, 0),
                Direction::Left => (-1, -1),
                Direction::Unknown => (0, 0),
            };
            let x = teleporter_position.x + offset_x;
            let y = teleporter_position.y + offset_y;

            if y > 0 && !self.hitmap[y.max(0) as usize][x.max(0) as usize] {
                entity.body_mut().frame.x = x;
                entity.body_mut().frame.y = y;
                entity.body_mut().direction = *hero_direction;
            } else {
                entity.body_mut().frame.x = x;
                entity.body_mut().frame.y = y + 2;
                entity.body_mut().direction = Direction::Down;
            }
        } else {
            entity.center_in(&self.bounds);
            entity.body_mut().direction = *hero_direction;
        }
        entity.immobilize_for_seconds(0.2);        
        self.add_entity(Box::new(entity));
    }    
}

