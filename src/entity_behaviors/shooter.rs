use crate::game_engine::{entity::Entity, entity_factory::EntityFactory, world::Game, behaviors::EntityBehavior};

#[derive(Debug)]
pub struct Shooter;

impl Shooter {
    pub fn new() -> Self {
        Self {}
    }
}

impl EntityBehavior for Shooter {
    fn update(&self, entity_id: &u32, world: &mut Game, time_since_last_update: f32) {
        let entity = world.entities.get_mut(entity_id).unwrap();
        if !entity.species.is_shooter() {
            return; 
        }

        entity.time_to_next_shot -= time_since_last_update;
        
        if entity.time_to_next_shot <= 0.0 {
            entity.time_to_next_shot = entity.species.time_between_shots - entity.time_to_next_shot;
            let bullet = self.build_bullet(&world.entity_factory, entity);
            world.add_entity(bullet);
        }
    }
}

impl Shooter {
    fn build_bullet(&self, entity_factory: &EntityFactory, parent: &Entity) -> Entity {
        let mut entity = entity_factory.build("towerdart");
        entity.parent_id = parent.id;
        entity.direction = parent.direction;
        entity.frame.x = parent.frame.x + (parent.frame.width - entity.frame.width) / 2.0;
        entity.frame.y = parent.frame.y + (parent.frame.height - entity.frame.height) / 2.0;
        return entity;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_shoot_stuff() {
        // ...
    }
}