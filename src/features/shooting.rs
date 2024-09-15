use crate::{entities::{known_species::SPECIES_KUNAI, species::species_by_id}, game_engine::{entity::Entity, inventory::{inventory_contains_species, remove_one_of_species_from_inventory}, state_updates::WorldStateUpdate, world::World}};

impl Entity {
    pub fn shoot_stuff(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
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
        bullet.reset_speed();

        vec![WorldStateUpdate::AddEntity(Box::new(bullet))]
    }
}
