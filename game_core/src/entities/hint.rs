use crate::{constants::{SPRITE_SHEET_AVATARS, SPRITE_SHEET_INVENTORY, SPRITE_SHEET_STATIC_OBJECTS}, game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, storage::{get_value_for_key, set_value_for_key, StorageKey}, world::World}, lang::localizable::LocalizableText, menus::toasts::{Toast, ToastImage}, utils::rect::IntRect};

use super::species::species_by_id;

impl Entity {
    pub fn setup_hint(&mut self, creative_mode: bool) {
        if creative_mode { 
            let species = species_by_id(self.species_id);
            self.sprite.sheet_id = SPRITE_SHEET_INVENTORY;
            self.sprite.frame.x = species.inventory_texture_offset.1;
            self.sprite.frame.y = species.inventory_texture_offset.0;
        } else {
            self.sprite.sheet_id = SPRITE_SHEET_STATIC_OBJECTS;
            self.sprite.frame.x = 4;
            self.sprite.frame.y = 2;
        }
    }

    pub fn update_hint(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {   
        if !world.creative_mode && world.is_hero_at(self.frame.x, self.frame.y) {
            self.hint_updates()    
        } else {
            vec![]
        }
    }

    fn hint_updates(&self) -> Vec<WorldStateUpdate> {
        if self.is_consumable && self.has_been_read() || self.contents.is_none() {
            vec![]
        } else {
            self.set_read();
            vec![self.toast()]
        }        
    }

    fn toast(&self) -> WorldStateUpdate {
        let hint = self.key().localized();
        WorldStateUpdate::EngineUpdate(EngineStateUpdate::Toast(            
            Toast::goddess_message(hint)
        ))
    }

    fn key(&self) -> String {
        self.contents.clone().unwrap_or("".to_owned())
    }

    fn has_been_read(&self) -> bool {
        has_hint_been_read(&self.key())
    }

    fn set_read(&self) {
        set_hint_read(&self.key())
    }
}

impl StorageKey {
    fn hint_read(hint: &str) -> String {
        format!("hint.read.{}", hint)
    }
}

fn set_hint_read(hint: &str) {
    set_value_for_key(&StorageKey::hint_read(hint), 1);
}

fn has_hint_been_read(hint: &str) -> bool {
    if let Some(read) = get_value_for_key(&StorageKey::hint_read(hint)) {
        read == 1
    } else {
        false
    }
}

impl Toast {
    fn goddess_message(text: String) -> Self {
        Toast::important_with_image(
            text, 
            ToastImage::new(
                IntRect::new(0, 0, 2, 2), 
                SPRITE_SHEET_AVATARS, 
                3
            )
        )
    }
}