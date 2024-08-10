
use crate::{features::{animated_sprite::update_sprite, autoremove::remove_automatically, check_bullet_collisions::handle_collisions_for_bullet, linear_movement::move_linearly}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::EntityFactory, game::Game, game_state_update::GameStateUpdate}, impl_animated_entity, impl_embodied_entity};

#[derive(Debug)]
pub struct TowerDart {
    body: EntityBody
}

impl TowerDart {
    pub fn new(parent: &dyn Entity, entity_factory: &EntityFactory) -> Self {
        let mut body = entity_factory.build("towerdart");
        body.resize(10.0, 10.0);
        body.is_bullet = true;
        body.requires_collision_detection = true;
        body.dp = 60.0;
        body.hp = 100.0;
        body.parent_id = parent.id();
        body.direction = parent.body().direction;
        body.base_speed = 5.0;
        body.reset_speed();
        body.center_in(&parent.body().frame);
        
        Self {
            body
        }
    }
}

impl_embodied_entity!(TowerDart);
impl_animated_entity!(TowerDart);

impl Entity for TowerDart {
    fn update(&mut self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate> {
        let mut game_updates: Vec<GameStateUpdate> = vec![];
        move_linearly(self, time_since_last_update);
        update_sprite(self, time_since_last_update);
        game_updates.append(&mut handle_collisions_for_bullet(self, game));
        game_updates.append(&mut remove_automatically(self, game));
        game_updates
    }
}