use std::sync::{atomic::{AtomicU32, Ordering}, Once};

use raylib::math::{Rectangle, Vector2};

use crate::{constants::{BASE_ENTITY_SIZE, BASE_ENTITY_SPEED, SPRITE_NAME_MOVEMENT}, entity_capabilities::{linear_movement::LinearMovement, shooter::Shooter}, species::{species_parser::SpeciesParser, species_repository::SpeciesRepository}, sprites::{sprite::Sprite, sprite_set_builder::SpriteSetBuilder, sprites_repository::SpritesRepository}};

use super::{entity::Entity, entity_capability::EntityCapability};

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
        let species = self.species_repo.species(species_id.to_owned());
        let sprites = self.sprites_repo.sprites(species_id);

        let frame = Rectangle::new(
            50.0,
            50.0,
            BASE_ENTITY_SIZE * species.scale,
            BASE_ENTITY_SIZE * species.scale,
        );

        let mut entity = Entity {
            id: get_next_entity_id(),
            frame: frame,
            direction: Vector2::new(1.0, 0.0),
            speed: BASE_ENTITY_SPEED * species.speed,
            species: species_id.to_owned(),
            sprite_set: sprites.clone(),
            current_sprite: Sprite::new("".to_owned(), Vec::new(), 1.0),
            capabilities: self.capabilities(&species.capabilities),
            is_enemy: species.is_enemy,
        };
        entity.change_sprite(SPRITE_NAME_MOVEMENT);
        return entity;
    }

    fn capabilities(&self, names: &Vec<String>) -> Vec<Box<dyn EntityCapability>> {
        let mut capabilities: Vec<Box<dyn EntityCapability>> = vec![];

        if names.contains(&"LinearMovement".to_owned()) {
            capabilities.push(Box::new(LinearMovement::new()));
        }
        if names.contains(&"Shooter".to_owned()) {
            capabilities.push(Box::new(Shooter::new()));
        }
        return capabilities;
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::{TEST_ASSETS_PATHS, TEST_SPECIES_PATHS};

    use super::*;

    impl EntityFactory {
        pub fn test() -> EntityFactory { 
            return EntityFactory::new(
                TEST_SPECIES_PATHS.iter().map(|&s| s.to_string()).collect(), 
                TEST_ASSETS_PATHS.iter().map(|&s| s.to_string()).collect()
            );
        }
    }
}
