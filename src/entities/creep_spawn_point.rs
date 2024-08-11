use raylib::math::Vector2;

use crate::{features::{animated_sprite::update_sprite, autoremove::remove_automatically, linear_movement_within_game_bounds::move_linearly_within_bounds}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::EntityFactory, game::Game, game_state_update::GameStateUpdate}, impl_animated_entity, impl_embodied_entity};


#[derive(Debug)]
pub struct CreepSpawnPoint {
    body: EntityBody,
    last_spawn_time: f32,
    time_to_spawn: f32
}

impl CreepSpawnPoint {
    pub fn new(body: EntityBody) -> Self {
        Self { 
            body,
            last_spawn_time: 0.0,
            time_to_spawn: 2.0
        }
    }
}

impl_embodied_entity!(CreepSpawnPoint);
impl_animated_entity!(CreepSpawnPoint);

impl Entity for CreepSpawnPoint {
    fn update(&mut self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate> {
        let mut game_updates: Vec<GameStateUpdate> = vec![];
        move_linearly_within_bounds(self, &game.bounds, time_since_last_update);
        update_sprite(self, time_since_last_update);

        if game.total_elapsed_time - self.last_spawn_time > self.time_to_spawn {
            self.last_spawn_time = game.total_elapsed_time;
            game_updates.push(GameStateUpdate::AddEntity(self.build_creep(game)))
        }

        game_updates.append(&mut remove_automatically(self, game));
        game_updates
    }
}

impl CreepSpawnPoint {
    fn build_creep(&self, game: &Game) -> Box<dyn Entity> {
        let mut creep = game.entity_factory.build_creep();
        creep.center_in(&self.body().frame);
        creep.body_mut().direction = Vector2::new(1.0, 0.0);
        Box::new(creep)
    }
}

impl EntityFactory {
    pub fn build_creep_spawn_point(&self) -> CreepSpawnPoint {
        let mut body = self.build("creepspawnpoint");
        body.resize(50.0, 30.0);
        body.is_enemy = true;
        CreepSpawnPoint::new(body)
    }
}

impl Game {
    pub fn add_creep_spawn_point(&mut self) -> u32 {
        self.add_entity(Box::new(self.entity_factory.build_creep_spawn_point()))
    }
}