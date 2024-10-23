use crate::{constants::{WORLD_SIZE_COLUMNS, WORLD_SIZE_ROWS}, entities::{known_species::{SPECIES_GHOST, SPECIES_HOMUNCULUS, SPECIES_ZOMBIE}, species::species_by_id}, game_engine::{entity::Entity, state_updates::WorldStateUpdate, world::World}, maps::biome_tiles::Biome, utils::{directions::Direction, rect::IntRect}};
use rand::{rngs::ThreadRng, seq::SliceRandom};

pub struct CreepSpawner {
    time_to_next_spawn: f32,
    rng: ThreadRng,
}

impl CreepSpawner {
    pub fn new() -> Self {
        CreepSpawner {
            time_to_next_spawn: 1.0,
            rng: rand::thread_rng(),
        }
    }

    pub fn update(
        &mut self,
        world: &World,
        time_since_last_update: f32,
    ) -> Vec<WorldStateUpdate> {
        if !world.creep_spawn_enabled || world.creative_mode { 
            return vec![] 
        }
        self.time_to_next_spawn -= time_since_last_update;

        if self.time_to_next_spawn <= 0.0 {
            let hero_direction = world.cached_hero_props.direction;

            if let Some((x, y)) = self.next_creep_position(&hero_direction, world) {
                self.time_to_next_spawn = world.creep_spawn_interval;

                let mut entity = self.make_creep();
                entity.frame.x = x;
                entity.frame.y = y;

                return vec![WorldStateUpdate::AddEntity(Box::new(entity))];
            } else {
                return vec![];
            }
        }
        vec![]
    }

    fn make_creep(&mut self) -> Entity {
        let id = *[SPECIES_ZOMBIE, SPECIES_HOMUNCULUS, SPECIES_GHOST]
            .choose(&mut self.rng)
            .unwrap_or(&SPECIES_ZOMBIE);
        species_by_id(id).make_entity()
    }

    fn next_creep_position(
        &mut self,
        hero_direction: &Direction,
        world: &World,
    ) -> Option<(i32, i32)> {
        let IntRect { x, y, w, h } = world.visible_bounds;

        let mut possible_positions = Vec::new();

        match hero_direction {
            Direction::Up => {
                let spawn_y = y;
                for spawn_x in x..(x + w) {
                    if self.is_valid_spawn_position(spawn_x, spawn_y, world) {
                        possible_positions.push((spawn_x, spawn_y));
                    }
                }
            }
            Direction::Down => {
                let spawn_y = y + h - 1;
                for spawn_x in x..(x + w) {
                    if self.is_valid_spawn_position(spawn_x, spawn_y, world) {
                        possible_positions.push((spawn_x, spawn_y));
                    }
                }
            }
            Direction::Left => {
                let spawn_x = x;
                for spawn_y in y..(y + h) {
                    if self.is_valid_spawn_position(spawn_x, spawn_y, world) {
                        possible_positions.push((spawn_x, spawn_y));
                    }
                }
            }
            Direction::Right => {
                let spawn_x = x + w - 1;
                for spawn_y in y..(y + h) {
                    if self.is_valid_spawn_position(spawn_x, spawn_y, world) {
                        possible_positions.push((spawn_x, spawn_y));
                    }
                }
            }
            _ => {
                for spawn_x in x..(x + w) {
                    for spawn_y in y..(y + h) {
                        if self.is_valid_spawn_position(spawn_x, spawn_y, world) {
                            possible_positions.push((spawn_x, spawn_y));
                        }
                    }
                }
            }
        }

        if !possible_positions.is_empty() {
            let &(spawn_x, spawn_y) = possible_positions.choose(&mut self.rng).unwrap();
            Some((spawn_x, spawn_y))
        } else {
            None
        }
    }

    fn is_valid_spawn_position(&self, x: i32, y: i32, world: &World) -> bool {
        if x < 0 || y < 0 || x >= WORLD_SIZE_COLUMNS as i32 || y >= WORLD_SIZE_ROWS as i32 {
            return false;
        }
        let x_usize = x as usize;
        let y_usize = y as usize;
        let biome_tile = &world.biome_tiles.tiles[y_usize][x_usize];
        !matches!(biome_tile.tile_type, Biome::Nothing | Biome::Water)
    }
}
