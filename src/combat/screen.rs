use raylib::color::Color;

use crate::{entities::species::SPECIES_NONE, game_engine::{entity::Entity, keyboard_events_provider::KeyboardEventsProvider}, lang::localizable::LocalizableText, menus::menu::{Menu, MenuItem, MenuUpdate, MENU_BORDERS_TEXTURES}, text, texture, ui::components::{padding, with_textured_border, Spacing, Typography, View}, utils::vector::Vector2d, vstack, zstack};

pub struct FightScreen {
    pub enemy: Entity,
    pub is_open: bool,
    player_options: Menu<CombatOption>,
    enemy_hp_left: i32,
    enemy_hp_total: i32,
    player_hp_left: i32,
    player_hp_total: i32,
}

#[derive(Debug, Clone)]
enum CombatOption {
    Attack,
    Inventory,
    Run
}

impl MenuItem for CombatOption {
    fn title(&self) -> String {
        match self {
            CombatOption::Attack => "combat.options.attack".localized(),
            CombatOption::Inventory => "combat.options.inventory".localized(),
            CombatOption::Run => "combat.options.run".localized(),
        }
    }
}

impl FightScreen {
    pub fn new() -> Self {
        let mut player_options = Menu::<CombatOption>::empty();
        player_options.uses_backdrop = false;
        player_options.items = vec![
            CombatOption::Attack,
            CombatOption::Inventory,
            CombatOption::Run,
        ];

        Self {
            enemy: SPECIES_NONE.make_entity(),
            is_open: false,
            player_options,
            enemy_hp_left: 90,
            enemy_hp_total: 100,
            player_hp_left: 64,
            player_hp_total: 120,
        }
    }

    pub fn show(&mut self, enemy: &Entity) {
        self.enemy = enemy.clone();
        self.is_open = true;
        self.player_options.show();
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> MenuUpdate {
        self.enemy.sprite.update(time_since_last_update);
        self.player_options.update(keyboard, time_since_last_update)
    }

    pub fn enemy_avatar_ui(&self) -> View {
        texture!(
            self.enemy.sprite_sheet(), 
            self.enemy.texture_source_rect(), 
            Vector2d::new(self.enemy.frame.w as f32, self.enemy.frame.h as f32).scaled(4.0)
        )
    }    

    pub fn player_avatar_ui(&self) -> View {
        text!(Typography::Regular, "Player Avater".to_owned())
    }    

    pub fn battle_info_ui(&self) -> View {
        text!(Typography::Regular, "Battle Info".to_owned())
    }    

    pub fn player_options_ui(&self) -> View {
        self.player_options.ui()
    } 

    pub fn enemy_life_ui(&self) -> View {
        self.render_hp(&self.enemy.name, self.enemy_hp_left, self.enemy_hp_total)
    }

    pub fn player_life_ui(&self) -> View {
        self.render_hp("you", self.player_hp_left, self.player_hp_total)
    }    

    fn render_hp(&self, name: &str, hp_left: i32, hp_total: i32) -> View {
        let value = hp_left as f32 / hp_total as f32;
        let foreground = if value > 0.1 { Color::GREEN } else { Color::RED };

        padding(
            Spacing::LG,
            with_textured_border(
                MENU_BORDERS_TEXTURES, 
                zstack!(
                    Spacing::MD,
                    Color::BLACK,
                    vstack!(
                        Spacing::MD,
                        text!(Typography::Regular, name.localized()),
                        View::ProgressBar { foreground, background: Color::BLACK.alpha(0.0), value }
                    )
                )
            )
        )
    }   
}