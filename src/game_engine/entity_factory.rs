use std::sync::{atomic::{AtomicU32, Ordering}, Once};

use raylib::math::{Rectangle, Vector2};

use crate::{constants::{ANIMATION_NAME_FRONT, BASE_ENTITY_SIZE, BASE_ENTITY_SPEED, NO_PARENT}, species::{species_parser::SpeciesParser, species_repository::SpeciesRepository}, sprites::{sprite_set_builder::SpriteSetBuilder, sprites_repository::SpritesRepository}};

use super::entity::Entity;

static INIT: Once = Once::new();
static mut NEXT_ENTITY_INDEX: Option<AtomicU32> = None;

fn get_next_entity_id() -> u32 {
    let counter = unsafe {
        INIT.call_once(|| {
            NEXT_ENTITY_INDEX = Some(AtomicU32::new(1000));
        });
        NEXT_ENTITY_INDEX.as_ref().expect("Counter is not initialized")
    };
    counter.fetch_add(1, Ordering::SeqCst);
    return counter.load(Ordering::SeqCst);
}

#[derive(Debug)]
pub struct EntityFactory {
    species_repo: SpeciesRepository,
    sprites_repo: SpritesRepository
}

impl EntityFactory {
    pub fn new(species_paths: Vec<String>, assets_paths: Vec<String>) -> Self {
        let sprite_set_builder = SpriteSetBuilder::new();
        let species_parser = SpeciesParser::new();
        let mut species_repo = SpeciesRepository::new(species_parser);
        let mut sprites_repo = SpritesRepository::new(sprite_set_builder);

        species_repo.setup(&species_paths);
        sprites_repo.setup(&assets_paths);

        Self {
            species_repo,
            sprites_repo,
        }
    }

    pub fn build(&self, species_id: &str) -> Entity {
        let species = self.species_repo.species(&species_id.to_owned());
        let sprites = self.sprites_repo.sprites(&species_id.to_owned());

        let frame = Rectangle::new(
            50.0,
            50.0,
            BASE_ENTITY_SIZE * species.scale,
            BASE_ENTITY_SIZE * species.scale,
        );

        let entity_id = get_next_entity_id();
        let direction = Vector2::new(0.0, 0.0);
        let speed = BASE_ENTITY_SPEED * species.speed;
        let time_between_shots = 1.0 / (species.bullets_per_minute / 60.0);

        return Entity {
            id: entity_id,
            parent_id: NO_PARENT,
            frame: frame,
            direction: direction,
            speed: speed,
            hp: species.hp,
            dp: species.dp,
            species: species_id.to_owned(),
            sprite_set: sprites.clone(),
            current_sprite: sprites.sprite(ANIMATION_NAME_FRONT),
            sprite_invalidated: false,
            z_index: species.z_index,
            is_enemy: species.is_enemy,
            is_shooter: species.is_shooter,
            is_bullet: species.is_bullet,
            time_between_shots: time_between_shots,
            time_to_next_shot: time_between_shots,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::{ASSETS_PATH, SPECIES_PATH}, utils::file_utils::list_files};

    use super::*;

    impl EntityFactory {
        pub fn test() -> EntityFactory { 
            return EntityFactory::new(
                list_files(SPECIES_PATH, "json"), 
                list_files(ASSETS_PATH, "png")
            );
        }
    }
}
