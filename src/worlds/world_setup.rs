use crate::{constants::{WORLD_SIZE_COLUMNS, WORLD_SIZE_ROWS}, entities::{known_species::SPECIES_HERO, species::make_entity_by_species}, game_engine::{storage::save_pressure_plate_states, world::World}, utils::directions::Direction};

impl World {
    pub fn setup(&mut self, source: u32, hero_direction: &Direction, original_x: i32, original_y: i32) {
        self.update_tiles_hitmap();
        self.update_hitmaps();
        save_pressure_plate_states(self);

        let (requires_offset, destination_x, destination_y) = self.destination_x_y(source, original_x, original_y);        
        let mut entity = make_entity_by_species(SPECIES_HERO);

        let (offset_x, offset_y): (i32, i32) = if !requires_offset {
            (0, 0)
        } else {
            match hero_direction {
                Direction::Up => (0, -2),
                Direction::Right => (1, -1),
                Direction::Down => (0, 0),
                Direction::Left => (-1, -1),
                Direction::Unknown => (0, 0),
                Direction::Still => (0, 0),
            }
        };
        let x = destination_x + offset_x;
        let y = destination_y + offset_y;

        if y > 0 && !self.hitmap[y.max(0) as usize][x.max(0) as usize] {
            entity.frame.x = x;
            entity.frame.y = y;
            entity.direction = *hero_direction;
        } else {
            entity.frame.x = x;
            entity.frame.y = y + 2;
            entity.direction = Direction::Down;
        }
        
        entity.immobilize_for_seconds(0.2);        
        self.add_entity(entity);
    }    

    pub fn set_creative_mode(&mut self, enabled: bool) {
        self.creative_mode = enabled;
        self.entities.borrow_mut().iter_mut().for_each(|e| e.setup(enabled));
    }

    fn destination_x_y(&self, source: u32, original_x: i32, original_y: i32) -> (bool, i32, i32) {
        if original_x == 0 && original_y == 0 {            
            if let Some(teleporter_position) = self.find_teleporter_for_destination(source) {
                (true, teleporter_position.x, teleporter_position.y)
            } else {
                (true, WORLD_SIZE_COLUMNS as i32 / 2, WORLD_SIZE_ROWS as i32 / 2)
            }
        } else {
            (false, original_x, original_y)
        }
    }
}

