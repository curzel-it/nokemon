use std::sync::{atomic::{AtomicU32, Ordering}, Once};

use raylib::math::{Rectangle, Vector2};

use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT}, sprites::{sprite::Sprite, sprite_set_builder::SpriteSetBuilder, sprites_repository::SpritesRepository}};

use super::entity_body::EntityBody;

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
    counter.load(Ordering::SeqCst)
}

#[derive(Debug)]
pub struct EntityFactory {
    sprites_repo: SpritesRepository
}

impl EntityFactory {
    pub fn new(assets_paths: Vec<String>) -> Self {
        let sprite_set_builder = SpriteSetBuilder::new();
        let mut sprites_repo = SpritesRepository::new(sprite_set_builder);

        sprites_repo.setup(&assets_paths);

        Self {
            sprites_repo,
        }
    }

    pub fn build(&self, species_id: &str) -> EntityBody {
        let sprites = self.sprites_repo.sprites(&species_id.to_owned());

        EntityBody {
            id: get_next_entity_id(),
            parent_id: NO_PARENT,
            frame: Rectangle::new(0.0, 0.0, 50.0, 50.0),
            direction: Vector2::new(0.0, 0.0),
            current_speed: 1.0,
            base_speed: 1.0,
            hp: 100.0,
            dp: 0.0,
            sprite_set: sprites.clone(),
            current_sprite: Sprite::empty(),
            sprite_invalidated: true,
            time_to_next_shot: 1000.0,
            time_between_shots: 1000.0,
            creation_time: 0.0,
            requires_collision_detection: true,
            is_rigid: true,
            z_index: 0,
            is_ally: false,
            is_bullet: false,
            lifespan: INFINITE_LIFESPAN,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::ASSETS_PATH, utils::file_utils::list_files};

    use super::*;

    impl EntityFactory {
        pub fn test() -> EntityFactory { 
            EntityFactory::new(
                list_files(ASSETS_PATH, "png")
            )
        }
    }
}
