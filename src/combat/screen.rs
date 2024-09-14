use crate::{entities::species::SPECIES_NONE, game_engine::entity::Entity, text, ui::components::{TextStyle, View}};

pub struct FightScreen {
    pub enemy: Entity,
    pub is_open: bool
}

impl FightScreen {
    pub fn new() -> Self {
        Self {
            enemy: SPECIES_NONE.make_entity(),
            is_open: false
        }
    }

    pub fn show(&mut self, enemy: &Box<Entity>) {
        self.enemy = *enemy.clone();
        self.is_open = true;
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        // ...
    }

    pub fn ui(&self) -> View {
        text!(TextStyle::Regular, "Fight screen".to_owned())
    }

    pub fn enemy_life_ui(&self) -> View {
        text!(TextStyle::Regular, "Enemy Life".to_owned())
    }

    pub fn player_life_ui(&self) -> View {
        text!(TextStyle::Regular, "Player Life".to_owned())
    }    

    pub fn player_options_ui(&self) -> View {
        text!(TextStyle::Regular, "Plater Options".to_owned())
    }    

    pub fn enemy_avatar_ui(&self) -> View {
        text!(TextStyle::Regular, "Enemy Avatar".to_owned())
    }    

    pub fn player_avatar_ui(&self) -> View {
        text!(TextStyle::Regular, "Player Avater".to_owned())
    }    

    pub fn battle_info_ui(&self) -> View {
        text!(TextStyle::Regular, "Battle Info".to_owned())
    }    
}