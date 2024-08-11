use std::collections::HashMap;

use super::{sprite_set::SpriteSet, sprite_set_builder::SpriteSetBuilder};

#[derive(Debug)]
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
        self.sprite_sets = self.builder.sprite_sets(png_paths);
    }

    pub fn sprites(&self, species_id: &String) -> SpriteSet {
        let sprites = self.sprite_sets.get(species_id);
        match sprites {
            Some(sprites) => sprites.clone(),
            None => SpriteSet::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::{ANIMATION_NAME_FRONT, ASSETS_PATH, MISSING_SPRITE};
    use crate::utils::file_utils::list_files;

    use super::*;

    #[test]
    fn can_load_sprites_for_red() {
        let builder = SpriteSetBuilder::new();
        let mut sprites_repo = SpritesRepository::new(builder);
        
        let all_assets = list_files(ASSETS_PATH, "png");
        sprites_repo.setup(&all_assets);

        let sprite_set = sprites_repo.sprites(&"red".to_owned());
        assert!(sprite_set.sprite("walknw").number_of_frames() > 1);
        assert!(sprite_set.sprite("stillnw").number_of_frames() == 1);
    }

    #[test]
    fn can_load_sprites_for_tower() {
        let builder = SpriteSetBuilder::new();
        let mut sprites_repo = SpritesRepository::new(builder);
        
        let all_assets = list_files(ASSETS_PATH, "png");
        sprites_repo.setup(&all_assets);

        let sprite_set = sprites_repo.sprites(&"tower".to_owned());
        let sprite = sprite_set.sprite(ANIMATION_NAME_FRONT);
        let number_of_frames = sprite.number_of_frames();
        assert!(number_of_frames > 1);
    }

    #[test]
    fn can_fallback_on_front_sprite_when_directional_walk_not_available() {
        let builder = SpriteSetBuilder::new();
        let mut sprites_repo = SpritesRepository::new(builder);
        
        let all_assets = list_files(ASSETS_PATH, "png");
        sprites_repo.setup(&all_assets);

        let sprite_set = sprites_repo.sprites(&"tower".to_owned());
        let sprite = sprite_set.sprite("walkn");
        let number_of_frames = sprite.number_of_frames();
        assert!(number_of_frames > 1);
    }

    #[test]
    fn can_fallback_on_front_sprite_when_walk_not_available() {
        let builder = SpriteSetBuilder::new();
        let mut sprites_repo = SpritesRepository::new(builder);
        
        let all_assets = list_files(ASSETS_PATH, "png");
        sprites_repo.setup(&all_assets);

        let sprite_set = sprites_repo.sprites(&"tower".to_owned());
        let sprite = sprite_set.sprite("walk");
        let number_of_frames = sprite.number_of_frames();
        assert!(number_of_frames > 1);
    }

    #[test]
    fn can_load_default_for_missing_species() {
        let builder = SpriteSetBuilder::new();
        let mut sprites_repo = SpritesRepository::new(builder);
        
        let all_assets = list_files(ASSETS_PATH, "png");
        sprites_repo.setup(&all_assets);
            
        let sprite_set = sprites_repo.sprites(&"non existing".to_owned());
        let sprite = sprite_set.sprite(ANIMATION_NAME_FRONT);
        assert!(sprite.number_of_frames() == 1);
        assert_eq!(sprite.current_frame().clone(), MISSING_SPRITE.to_owned());
    }
}
