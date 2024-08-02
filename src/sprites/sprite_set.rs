use std::{collections::HashMap, fmt::Debug};

use crate::constants::{ANIMATIONS_FPS, MISSING_SPRITE, ANIMATION_NAME_FRONT};

use super::sprite::Sprite;

#[derive(Clone)]
pub struct SpriteSet {
    animations: HashMap<String, Vec<String>>,
}

impl SpriteSet {
    pub fn new(animations: HashMap<String, Vec<String>>) -> Self {
        SpriteSet { animations }
    }

    pub fn default() -> Self {
        SpriteSet {
            animations: HashMap::new(),
        }
    }

    pub fn sprite_frames(&self, animation_name: &str) -> Vec<String> {
        match self.animations.get(&animation_name.to_owned()) {
            Some(frames) => frames.clone(),
            None => {
                if animation_name != ANIMATION_NAME_FRONT {
                    return self.sprite_frames(ANIMATION_NAME_FRONT);
                } else {
                    return vec![MISSING_SPRITE.to_owned()]
                }
            }
        }
    }

    pub fn sprite(&self, animation_name: &str) -> Sprite {
        let frames = self.sprite_frames(animation_name);
        Sprite::new(animation_name.to_owned(), frames, ANIMATIONS_FPS)
    }
}

impl Debug for SpriteSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut names: Vec<&String> = self.animations.keys().collect();
        names.sort();

        return f.debug_struct("SpriteSet")
            .field("animations", &names)
            .finish();
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::{ANIMATION_NAME_FRONT, ANIMATION_NAME_MOVEMENT_E, ANIMATION_NAME_MOVEMENT_N, ANIMATION_NAME_MOVEMENT_NE, ANIMATION_NAME_MOVEMENT_NO, ANIMATION_NAME_MOVEMENT_O, ANIMATION_NAME_MOVEMENT_S, ANIMATION_NAME_MOVEMENT_SE, ANIMATION_NAME_MOVEMENT_SO};

    use super::*;

    fn generate_sprite_names(base_name: &str, count: u32) -> Vec<String> {
        let mut names = Vec::new();
        for i in 0..count {
            names.push(format!("{}-{}", base_name, i));
        }
        names
    }

    #[test]
    fn can_reference_standard_sprites() {
        let animations: HashMap<String, Vec<String>> = [
            (ANIMATION_NAME_MOVEMENT_N.to_string(), generate_sprite_names(ANIMATION_NAME_MOVEMENT_N, 3)),
            (ANIMATION_NAME_MOVEMENT_NE.to_string(), generate_sprite_names(ANIMATION_NAME_MOVEMENT_NE, 3)),
            (ANIMATION_NAME_MOVEMENT_E.to_string(), generate_sprite_names(ANIMATION_NAME_MOVEMENT_E, 3)),
            (ANIMATION_NAME_MOVEMENT_SE.to_string(), generate_sprite_names(ANIMATION_NAME_MOVEMENT_SE, 3)),
            (ANIMATION_NAME_MOVEMENT_S.to_string(), generate_sprite_names(ANIMATION_NAME_MOVEMENT_S, 3)),
            (ANIMATION_NAME_MOVEMENT_SO.to_string(), generate_sprite_names(ANIMATION_NAME_MOVEMENT_SO, 3)),
            (ANIMATION_NAME_MOVEMENT_O.to_string(), generate_sprite_names(ANIMATION_NAME_MOVEMENT_O, 3)),
            (ANIMATION_NAME_MOVEMENT_NO.to_string(), generate_sprite_names(ANIMATION_NAME_MOVEMENT_NO, 3)),
            (ANIMATION_NAME_FRONT.to_string(), generate_sprite_names(ANIMATION_NAME_FRONT, 3)),
        ].iter().cloned().collect();

        let sprite_set = SpriteSet::new(animations);

        assert_eq!(sprite_set.sprite_frames(ANIMATION_NAME_MOVEMENT_N)[0], "walk_n-0");
        assert_eq!(sprite_set.sprite_frames(ANIMATION_NAME_FRONT)[0], "front-0");
    }

    #[test]
    fn can_reference_animations() {
        let mut animations = HashMap::new();
        animations.insert("jump".to_string(), generate_sprite_names("jump", 5));
        animations.insert("run".to_string(), generate_sprite_names("run", 5));
        animations.insert("slide".to_string(), generate_sprite_names("slide", 5));

        let sprite_set = SpriteSet::new(animations);

        assert_eq!(sprite_set.sprite_frames("jump")[0], "jump-0");
        assert_eq!(sprite_set.sprite_frames("run")[0], "run-0");
        assert_eq!(sprite_set.sprite_frames("slide")[0], "slide-0");
    }
}
