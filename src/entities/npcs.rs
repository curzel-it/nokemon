use crate::{game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, utils::directions::{direction_between_rects, Direction}};

pub type NpcId = u32;

impl Entity {
    pub fn setup_npc(&mut self) {
        self.setup_patrol();
        self.update_sprite_for_current_state();
    }

    pub fn update_npc(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {  
        self.update_sprite_for_current_state();
        self.handle_patrol();
        
        if !world.creative_mode {
            self.move_linearly(world, time_since_last_update);
            
            if self.melee_attacks_hero {
                self.search_for_hero(world);
                let updates = self.handle_melee_attack(world);
                
                if !updates.is_empty() {
                    return updates
                }
            }
        }

        if world.is_hero_around_and_on_collision_with(&self.frame) {
            self.direction = direction_between_rects(&self.frame, &world.cached_hero_props.hittable_frame);

            if world.creative_mode {
                let vec = vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowEntityOptions(
                            Box::new(self.clone())
                        )
                    )
                ];
                return vec;  
            } else if let Some(dialogue) = self.next_dialogue() {
                self.demands_attention = false;

                return vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowDialogue(
                            self.id, self.name.clone(), dialogue,
                        )
                    )
                ];
            }             
        }  
        vec![]
    }

    fn search_for_hero(&mut self, world: &World) {
        if self.offset.x != 0.0 || self.offset.y != 0.0 {
            return
        }
        if self.is_hero_in_line_of_sight(world) {
            self.change_direction_towards_hero(world);
        } else if self.is_obstacle_in_direction(&world.hitmap, self.direction) {
            self.pick_next_direction(&world.hitmap);
        }
    }

    fn is_hero_in_line_of_sight(&self, world: &World) -> bool {
        let hero = &world.cached_hero_props.hittable_frame;        
        let npc = &self.frame;
        let npc_y = self.frame.y + if self.frame.h > 1 { 1 } else { 0 };

        if npc.x == hero.x {
            let min_y = npc_y.min(hero.y);
            let max_y = npc_y.max(hero.y);
            for y in (min_y + 1)..max_y {
                if world.hitmap[y as usize][npc.x as usize] {
                    return false;
                }
            }
            true
        } else if npc_y == hero.y || self.frame.y == hero.y {
            let min_x = npc.x.min(hero.x);
            let max_x = npc.x.max(hero.x);
            for x in (min_x + 1)..max_x {
                if world.hitmap[npc_y as usize][x as usize] {
                    return false;
                }
            }
            true
        } else {
            false
        }

    }

    fn change_direction_towards_hero(&mut self, world: &World) {
        let hero = &world.cached_hero_props.hittable_frame;
        let npc = &self.frame;

        if hero.x == npc.x {
            if hero.y < npc.y {
                self.direction = Direction::Up;
            } else {
                self.direction = Direction::Down
            }
        } else if hero.y == npc.y || hero.y == npc.y + 1 {
            if hero.x > npc.x {
                self.direction = Direction::Right;
            } else {
                self.direction = Direction::Left
            }
        }
    }

    fn is_obstacle_in_direction(&self, hitmap: &[Vec<bool>], direction: Direction) -> bool {
        let (next_dx, next_dy) = direction.as_col_row_offset();
        let next_x = self.frame.x + next_dx;
        let next_y = self.frame.y + next_dy + if self.frame.h > 1 { 1 } else { 0 };

        if next_x < 0
            || next_x >= hitmap[0].len() as i32
            || next_y < 0
            || next_y >= hitmap.len() as i32 {
            return true; 
        }

        hitmap[next_y as usize][next_x as usize]
    }

    fn pick_next_direction(&mut self, hitmap: &[Vec<bool>]) {
        let directions = [
            self.direction.turn_right(),
            self.direction.turn_left(),
            self.direction.opposite(),
        ];

        for &dir in &directions {
            if !self.is_obstacle_in_direction(hitmap, dir) {
                self.direction = dir;
                break;
            }
        }
    }
}