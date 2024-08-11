use raylib::math::Vector2;

use crate::{constants::HERO_ENTITY_ID, features::{animated_sprite::update_sprite, autoremove::remove_automatically, keyboard_directions::set_direction_according_to_keyboard_state, linear_movement::move_linearly, shooter::{shoot_stuff, Shooter}}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::EntityFactory, world_state_update::WorldStateUpdate, world::World}, impl_animated_entity, impl_embodied_entity};

use super::surrounding_area_attack::SurroundingAreaAttack;

#[derive(Debug)]
pub struct Hero {
    body: EntityBody,
    time_to_next_shot: f32
}

impl Hero {
    pub fn new(body: EntityBody) -> Self {
        Self { 
            body,
            time_to_next_shot: 5.0
        }
    }
}

impl_embodied_entity!(Hero);
impl_animated_entity!(Hero);

impl Shooter for Hero {
    fn time_to_next_shot(&self) -> f32 {
        self.time_to_next_shot
    }
    
    fn inc_time_to_next_shot(&mut self, delta: f32) {
        self.time_to_next_shot += delta;
    }
    
    fn reset_time_to_next_shot(&mut self) {
        self.time_to_next_shot = self.body().time_between_shots;
    }
    
    fn create_bullet(&self, entity_factory: &EntityFactory) -> Box<dyn Entity> {
        Box::new(SurroundingAreaAttack::new(self, entity_factory))
    }
}

impl Entity for Hero {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        set_direction_according_to_keyboard_state(self, &world.keyboard_state);
        move_linearly(self, time_since_last_update);
        update_sprite(self, time_since_last_update);
        world_updates.append(&mut shoot_stuff(self, world, time_since_last_update));
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }
}

impl EntityFactory {
    pub fn build_hero(&self) -> Hero {
        let mut body = self.build("red");
        body.resize(15.0, 17.0);
        body.id = HERO_ENTITY_ID;
        body.time_to_next_shot = 5.0;
        body.time_between_shots = 5.0;
        body.base_speed = 3.0;
        body.is_ally = true;
        body.reset_speed();

        let mut hero = Hero::new(body);
        hero.body_mut().direction = Vector2::new(1.0, 0.0);            
        hero
    }
}