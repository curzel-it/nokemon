use std::sync::{atomic::{AtomicU32, Ordering}, Once};

use raylib::math::{Rectangle, Vector2};

use crate::{constants::{ANIMATION_NAME_FRONT, NO_PARENT, SCALE}, species::{self, species_parser::SpeciesParser, species_repository::SpeciesRepository}, sprites::{sprite::Sprite, sprite_set_builder::SpriteSetBuilder, sprites_repository::SpritesRepository}};

use super::{entity::Entity, entity_body::EntityBody, simple_entity::SimpleEntity};

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

    pub fn build_simple(&self, species_id: &str) -> Box<dyn Entity> {
        Box::new(SimpleEntity::new(self.build(species_id)))
    }

    pub fn build_simple_with_id(&self, species_id: &str, id: u32) -> Box<dyn Entity> {
        let mut body = self.build(species_id);
        body.id = id;
        return Box::new(SimpleEntity::new(body));
    }

    pub fn build(&self, species_id: &str) -> EntityBody {
        let species = self.species_repo.species(&species_id.to_owned());
        let sprites = self.sprites_repo.sprites(&species_id.to_owned());

        let frame = Rectangle::new(
            50.0,
            50.0,
            SCALE * species.width,
            SCALE * species.height,
        );

        let mut entity = EntityBody {
            id: get_next_entity_id(),
            parent_id: NO_PARENT,
            frame: frame,
            direction: Vector2::new(0.0, 0.0),
            speed: 0.0,
            hp: species.hp,
            dp: species.dp,
            sprite_set: sprites.clone(),
            current_sprite: Sprite::empty(),
            sprite_invalidated: true,
            time_to_next_shot: species.time_between_shots,
            species: species,
            creation_time: 0.0
        };
        entity.reset_speed();
        entity.set_animation(ANIMATION_NAME_FRONT);

        return entity;
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
