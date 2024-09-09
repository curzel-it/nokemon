use crate::{entities::{known_species::SPECIES_HERO, species::make_entity_by_species}, game_engine::world::World, utils::directions::Direction};

impl World {
    pub fn setup(&mut self, source: &u32, hero_direction: &Direction) {
        self.compute_hitmap();

        let mut entity = make_entity_by_species(SPECIES_HERO);
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
                entity.frame.x = x;
                entity.frame.y = y;
                entity.direction = *hero_direction;
            } else {
                entity.frame.x = x;
                entity.frame.y = y + 2;
                entity.direction = Direction::Down;
            }
        } else {
            entity.center_in(&self.bounds);
            entity.direction = *hero_direction;
        }
        entity.immobilize_for_seconds(0.2);        
        self.add_entity(entity);
    }    
}

