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
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        // ...
    }

    pub fn ui(&self) -> View {
        text!(TextStyle::Regular, "Fight screen".to_owned())
    }
}