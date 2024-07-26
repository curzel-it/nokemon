use std::collections::HashMap;

use super::{sprite_set::SpriteSet, sprite_set_builder::SpriteSetBuilder};

pub struct SpritesRepository {
    builder: SpriteSetBuilder,
    sprite_sets: HashMap<String, SpriteSet>,
}

impl SpritesRepository {
    pub fn new(builder: SpriteSetBuilder) -> Self {
        Self {
            builder,
            sprite_sets: HashMap::new(),
        }
    }

    pub fn setup(&mut self, png_paths: &Vec<String>) {
        self.sprite_sets = self.builder.sprite_sets(&png_paths);
    }

    pub fn sprites(&self, species_id: &str) -> SpriteSet {
        let sprites = self.sprite_sets.get(species_id);
        return match sprites {
            Some(sprites) => sprites.clone(),
            None => SpriteSet::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::{ASSETS_PATH, MISSING_SPRITE, SPRITE_NAME_FRONT};
    use crate::utils::file_utils::list_files;

    use super::*;

    #[test]
    fn can_load_sprites_for_existing_species() {
        let builder = SpriteSetBuilder::new();
        let mut sprites_repo = SpritesRepository::new(builder);
        
        let all_assets = list_files(ASSETS_PATH, "png");
        sprites_repo.setup(&all_assets);

        let sprite_set = sprites_repo.sprites("ape");
        let number_of_frames = sprite_set.sprite(SPRITE_NAME_FRONT, 1.0).number_of_frames();
        assert!(number_of_frames > 1);
    }

    #[test]
    fn can_load_default_for_missing_species() {
        let builder = SpriteSetBuilder::new();
        let mut sprites_repo = SpritesRepository::new(builder);
        
        let all_assets = list_files(ASSETS_PATH, "png");
        sprites_repo.setup(&all_assets);
            
        let sprite_set = sprites_repo.sprites("non existing");
        let sprite = sprite_set.sprite(SPRITE_NAME_FRONT, 1.0);
        assert!(sprite.number_of_frames() == 1);
        assert_eq!(sprite.current_frame().clone(), MISSING_SPRITE.to_owned());
    }
}
