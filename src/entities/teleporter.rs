use crate::{game_engine::{entity::Entity, inventory::inventory_contains, locks::LockType, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, lang::localizable::LocalizableText, utils::directions::Direction};

impl Entity {
    pub fn update_teleporter(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {   
        if world.is_hero_around_and_on_collision_with(&self.frame) {
            if world.creative_mode {
                return vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowEntityOptions(
                            self.name.clone(), self.id, self.entity_type
                        )
                    )
                ];
            }
        } 

        if self.should_teleport(world) {
            if self.lock_type != LockType::None {
                if inventory_contains(self.lock_type.key()) {
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
            Direction::Unknown => false
        }
    }

    fn engine_update_push_world(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::SwitchWorld(
                self.destination
            )
        )
    }

    fn show_locked_message(&self) -> WorldStateUpdate {
        let name = self.lock_type.localized_name().to_uppercase();
        
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::Toast(
                "teleporter.locked".localized().replace("%s", &name)
            )
        )
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