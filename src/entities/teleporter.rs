use crate::{game_engine::{entity::Entity, inventory::inventory_contains_species, locks::LockType, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, lang::localizable::LocalizableText, menus::toasts::Toast, utils::directions::Direction};

impl Entity {
    pub fn setup_teleporter(&mut self, creative_mode: bool) {
        self.sprite.frame.y = if creative_mode { 5 } else { 6 };
    }

    pub fn update_teleporter(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {   
        if world.creative_mode && world.is_hero_around_and_on_collision_with(&self.frame) {
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        Box::new(self.clone())
                    )
                )
            ];
        } 

        if self.should_teleport(world) {
            if !world.creative_mode && self.lock_type != LockType::None {
                if inventory_contains_species(self.lock_type.key()) {
                    vec![self.show_unlock_confirmation()]
                } else {
                    vec![self.show_locked_message()]
                }                
            } else {
                vec![self.engine_update_push_world()]
            }
        } else {
            vec![]
        }        
    }

    fn should_teleport(&self, world: &World) -> bool {
        let hero = world.cached_hero_props.hittable_frame;
        let hero_direction = world.cached_hero_props.direction;
        let hero_speed = world.cached_hero_props.speed;

        if !world.is_any_arrow_key_down { return false }
        if hero_speed <= 0.0 { return false }

        match hero_direction {
            Direction::Up => hero.x == self.frame.x && hero.y == self.frame.y + 1,
            Direction::Right => hero.y == self.frame.y && hero.x == self.frame.x - 1,
            Direction::Down => hero.x == self.frame.x && hero.y == self.frame.y - 1,
            Direction::Left => hero.y == self.frame.y && hero.x == self.frame.x + 1,
            Direction::Unknown => false,
            Direction::Still => false,
        }
    }

    fn engine_update_push_world(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::Teleport(
                self.destination.clone().unwrap_or_default()
            )
        )
    }

    fn show_locked_message(&self) -> WorldStateUpdate {        
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::Toast(
                Toast::regular(self.locked_message())
            )
        )
    }

    fn locked_message(&self) -> String {
        if matches!(self.lock_type, LockType::Permanent) {
            "telepoter.locked.permanent".localized()
        } else {
            let name = self.lock_type.localized_name().to_uppercase();
            "teleporter.locked".localized().replace("%s", &name)
        }
    } 

    fn show_unlock_confirmation(&self) -> WorldStateUpdate {
        let name = self.lock_type.localized_name().to_uppercase();
        
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::Confirmation(
                "teleporter.unlock.title".localized(),
                "teleporter.unlock.message".localized().replace("%s", &name),
                vec![
                    WorldStateUpdate::ChangeLock(self.id, LockType::None),
                    WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame),
                    WorldStateUpdate::EngineUpdate(EngineStateUpdate::RemoveFromInventory(self.lock_type.key()))
                ]
            )
        )
    }
}