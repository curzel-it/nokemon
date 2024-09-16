use crate::{
    constants::CREEP_SPAWN_INTERVAL, entities::{known_species::{SPECIES_GHOST, SPECIES_ZOMBIE}, species::species_by_id}, game_engine::{entity::Entity, state_updates::WorldStateUpdate, world::World}, utils::{directions::Direction, rect::Rect}
};
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

pub struct CreepSpawner {
    time_to_next_spawn: f32,
    rng: ThreadRng,
}

impl CreepSpawner {
    pub fn new() -> Self {
        CreepSpawner {
            time_to_next_spawn: CREEP_SPAWN_INTERVAL,
            rng: rand::thread_rng()
        }
    }

    pub fn update(
        &mut self,
        world: &World,
        time_since_last_update: f32,
    ) -> Vec<WorldStateUpdate> {
        self.time_to_next_spawn -= time_since_last_update;

        if self.time_to_next_spawn <= 0.0 {
            self.time_to_next_spawn = CREEP_SPAWN_INTERVAL;

            let hero_direction = world.cached_hero_props.direction;
            let (x, y) = self.next_creep_position(&hero_direction, world);

            let mut entity = self.make_creep();
            entity.frame.x = x;
            entity.frame.y = y;

            return vec![WorldStateUpdate::AddEntity(Box::new(entity))];
        }

        vec![]
    }

    fn make_creep(&mut self) -> Entity {
        let id = *[SPECIES_ZOMBIE, SPECIES_GHOST]
            .choose(&mut self.rng)
            .unwrap_or(&SPECIES_ZOMBIE);
        species_by_id(id).make_entity()
    }

    fn next_creep_position(&mut self, hero_direction: &Direction, world: &World) -> (i32, i32) {
        let Rect { x, y, w, h } = world.visible_bounds;

        match hero_direction {
            Direction::Up => {
                let spawn_x = self.rng.gen_range(x..(x + w));
                let spawn_y = y;
                (spawn_x, spawn_y)
            }
            Direction::Down => {
                let spawn_x = self.rng.gen_range(x..(x + w));
                let spawn_y = y + h;
                (spawn_x, spawn_y)
            }
            Direction::Left => {
                let spawn_x = x;
                let spawn_y = self.rng.gen_range(y..(y + h));
                (spawn_x, spawn_y)
            }
            Direction::Right => {
                let spawn_x = x + w;
                let spawn_y = self.rng.gen_range(y..(y + h));
                (spawn_x, spawn_y)
            }
            _ => {
                let spawn_x = self.rng.gen_range(x..(x + w));
                let spawn_y = self.rng.gen_range(y..(y + h));
                (spawn_x, spawn_y)
            }
        }
    }
}
