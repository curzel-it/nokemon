use crate::{entities::{known_species::SPECIES_KUNAI, species::species_by_id}, game_engine::{entity::{Entity, EntityProps}, inventory::{inventory_contains_species, remove_one_of_species_from_inventory}, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, utils::rect::Rect};

impl Entity {
    pub fn update_hero(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {        
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        
        if !world.is_hero_on_slippery_surface() {
            self.update_direction_for_current_keys(world.direction_based_on_current_keys);
            self.update_sprite_for_current_direction();
        } else {
            self.update_sprite_for_direction_speed(self.direction, 0.0);
        }
        
        self.time_immobilized -= time_since_last_update;
        if self.time_immobilized <= 0.0 {
            self.move_linearly(world, time_since_last_update)
        }
        
        world_updates.push(self.cache_props());
        world_updates.push(self.move_camera_update());
        world_updates.append(&mut self.shoot_kunai(world, time_since_last_update));
        world_updates
    }

    fn cache_props(&self) -> WorldStateUpdate {
        WorldStateUpdate::CacheHeroProps(
            Box::new(self.props())
        )
    }

    fn props(&self) -> EntityProps {
        EntityProps {
            frame: self.frame,
            direction: self.direction,
            offset: self.offset,
            speed: self.current_speed,
            hittable_frame: Rect {
                x: self.frame.x,
                y: self.frame.y + 1,
                w: 1,
                h: 1,
            }
        }            
    }

    fn move_camera_update(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::CenterCamera(
                self.frame.x, 
                self.frame.y,
                self.offset
            )
        )
    }
    
    fn shoot_kunai(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        self.shooting_cooldown_remaining -= time_since_last_update;
        if self.shooting_cooldown_remaining > 0.0 {
            return vec![]
        }

        if !world.has_attack_key_been_pressed {
            return vec![]
        }

        if !inventory_contains_species(SPECIES_KUNAI) {
            return vec![]
        }

        self.shooting_cooldown_remaining = 0.1;
        remove_one_of_species_from_inventory(SPECIES_KUNAI);

        let mut bullet = species_by_id(SPECIES_KUNAI).make_entity();
        bullet.direction = world.cached_hero_props.direction;
        let (dx, dy) = bullet.direction.as_col_row_offset();
        bullet.frame = world.cached_hero_props.frame.offset(dx, dy).offset_y(1).with_h(1);
        bullet.parent_id = self.id;
        bullet.remaining_lifespan = 5.0;
        bullet.reset_speed();

        vec![WorldStateUpdate::AddEntity(Box::new(bullet))]
    }
}