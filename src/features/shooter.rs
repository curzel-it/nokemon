use crate::game_engine::{entity::Entity, entity_factory::EntityFactory, game::Game, game_state_update::GameStateUpdate, simple_entity::SimpleEntity};

pub trait Shooter: Entity {
    fn time_to_next_shot(&self) -> f32;
    fn inc_time_to_next_shot(&mut self, delta: f32);
    fn reset_time_to_next_shot(&mut self);
}

pub fn shoot_stuff(entity: &mut dyn Shooter, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate> {
    entity.inc_time_to_next_shot(-time_since_last_update);
    
    if entity.time_to_next_shot() <= 0.0 {
        entity.reset_time_to_next_shot();
        let bullet = build_bullet(&game.entity_factory, entity);
        return vec![GameStateUpdate::AddEntity(bullet)];
    }
    vec![]
}

fn build_bullet(entity_factory: &EntityFactory, parent: &dyn Shooter) -> Box<dyn Entity> {
    let mut body = entity_factory.build("towerdart");
    body.parent_id = parent.id();
    body.direction = parent.direction();
    body.center_in(&parent.frame());
    Box::new(SimpleEntity::new(body))
}