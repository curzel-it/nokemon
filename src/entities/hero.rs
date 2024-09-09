use crate::{game_engine::{entity::{Entity, EntityProps}, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, utils::rect::Rect};

impl Entity {
    pub fn update_hero(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {        
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        
        self.update_direction_for_current_keys(world.direction_based_on_current_keys);
        self.update_sprite_for_current_direction();
        
        world_updates.push(self.cache_props());
        world_updates.push(self.move_camera_update());
        world_updates
    }

    fn cache_props(&self) -> WorldStateUpdate {
        WorldStateUpdate::CacheHeroProps(
            self.props()           
        )
    }

    fn props(&self) -> EntityProps {
        EntityProps {
            frame: self.frame,
            direction: self.direction,
            speed: self.current_speed,
            hittable_frame: Rect {
                x: self.frame.x,
                y: self.frame.y + 1,
                w: 1,
                h: 1,
            }
        }            
    }

    fn move_camera_update(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::CenterCamera(
                self.frame.x, 
                self.frame.y,
                self.offset
            )
        )
    }
}