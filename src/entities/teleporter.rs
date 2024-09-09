use crate::{game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, utils::directions::Direction};

impl Entity {
    pub fn update_teleporter(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {      
        if self.should_teleport(world) {
            vec![self.engine_update_push_world()]
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
}