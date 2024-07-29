use std::sync::{atomic::{AtomicU32, Ordering}, Once};

use raylib::math::{Rectangle, Vector2};

use crate::{constants::{BASE_ENTITY_SIZE, BASE_ENTITY_SPEED, SPRITE_NAME_MOVEMENT}, entity_capabilities::{autoremove::Autoremove, linear_movement::LinearMovement, shooter::Shooter}, species::{species_model::SpeciesCapability, species_parser::SpeciesParser, species_repository::SpeciesRepository}, sprites::{sprite::Sprite, sprite_set_builder::SpriteSetBuilder, sprites_repository::SpritesRepository}};

use super::{entity::Entity, entity_capability::{EntityCapability, UnknownCapability}};

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

#[derive(Debug, Clone)]
pub struct EntityDescriptor {
    pub species: String,
    pub origin: Vector2,
    pub direction: Vector2
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
        let descriptor = EntityDescriptor {
            species: species_id.to_owned(),
            origin: Vector2::new(50.0, 50.0),
            direction: Vector2::new(1.0, 0.0)
        };
        return self.build_ex(&descriptor);
    }

    pub fn build_ex(&self, descriptor: &EntityDescriptor) -> Entity {
        let species_id = &descriptor.species;
        let species = self.species_repo.species(species_id);
        let sprites = self.sprites_repo.sprites(species_id);

        let frame = Rectangle::new(
            descriptor.origin.x,
            descriptor.origin.y,
            BASE_ENTITY_SIZE * species.scale,
            BASE_ENTITY_SIZE * species.scale,
        );

        let mut entity = Entity {
            id: get_next_entity_id(),
            frame: frame,
            direction: descriptor.direction,
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

    fn capabilities(&self, capabilities: &Vec<SpeciesCapability>) -> Vec<Box<dyn EntityCapability>> {
        let mut items: Vec<Box<dyn EntityCapability>> = vec![];

        for capability in capabilities {
            let new_item: Box<dyn EntityCapability> = match capability.name.as_str() {
                "Autoremove" => Box::new(Autoremove::new()),
                "LinearMovement" => Box::new(LinearMovement::new()),
                "Shooter" => Box::new(Shooter::new(
                    capability.get("rpm", 60.0)
                )),
                _ => Box::new(UnknownCapability::new(&capability.name)),
            };
            items.push(new_item);
        }
        return items;
    }
}

impl EntityDescriptor {
    pub fn for_species(species_id: &str) -> Self {
        Self {
            species: species_id.to_owned(),
            origin: Vector2::new(0.0, 0.0),
            direction: Vector2::new(1.0, 0.0),
        }
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
