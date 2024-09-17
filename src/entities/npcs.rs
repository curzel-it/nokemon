use crate::{constants::STEP_COMMITMENT_THRESHOLD, game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, utils::{directions::{direction_between_rects, Direction}, rect::Rect}};

pub type NpcId = u32;

const NO_DIALOG_SHOW_SHOP_INSTEAD: u32 = 3;

impl Entity {
    pub fn setup_npc(&mut self) {
        self.setup_patrol();
        self.update_sprite_for_current_direction();
    }

    pub fn update_npc(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {  
        self.update_sprite_for_current_direction();
        self.handle_patrol();
        self.move_linearly(world, time_since_last_update);
        
        if self.melee_attacks_hero {
            self.adjust_position_towards_hero(world);
            let updates = self.handle_melee_attack(world);
            
            if !updates.is_empty() {
                return updates
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
                if dialogue.id == NO_DIALOG_SHOW_SHOP_INSTEAD {
                    return vec![
                        WorldStateUpdate::EngineUpdate(
                            EngineStateUpdate::ShowShop
                        )
                    ];
                } else {
                    return vec![
                        WorldStateUpdate::EngineUpdate(
                            EngineStateUpdate::ShowDialogue(
                                self.id, self.name.clone(), dialogue,
                            )
                        )
                    ];
                }
            }             
        }  
        vec![]
    }

    fn adjust_position_towards_hero(&mut self, world: &World) {
        if self.offset.x < STEP_COMMITMENT_THRESHOLD && self.offset.y < STEP_COMMITMENT_THRESHOLD {
            self.adjust_position_towards(&world.cached_hero_props.hittable_frame, &world.hitmap)
        }        
    }

    fn adjust_position_towards(&mut self, hero: &Rect, obstacles: &[Vec<bool>]) {
        let x = self.frame.x;
        let y = self.frame.y.max(1) - if self.frame.h > 1 { 1 } else { 0 };
        let hero_x = hero.x;
        let hero_y = hero.y;
    
        let dx = hero_x - x;
        let dy = hero_y - y;
        let current_distance = dx.abs() + dy.abs();
    
        let mut possible_moves = Vec::new();
    
        if y >= obstacles.len() as i32 || x >= obstacles[0].len() as i32 {
            return;
        }

        if y > 0 && !obstacles[(y - 1) as usize][x as usize] {
            let new_distance = (hero_x - x).abs() + (hero_y - (y - 1)).abs();
            possible_moves.push((Direction::Up, new_distance));
        }
    
        if y + 1 < obstacles.len() as i32 && !obstacles[(y + 1) as usize][x as usize] {
            let new_distance = (hero_x - x).abs() + (hero_y - (y + 1)).abs();
            possible_moves.push((Direction::Down, new_distance));
        }
    
        if x > 0 && !obstacles[y as usize][(x - 1) as usize] {
            let new_distance = (hero_x - (x - 1)).abs() + (hero_y - y).abs();
            possible_moves.push((Direction::Left, new_distance));
        }
    
        if x + 1 < obstacles[0].len() as i32 && !obstacles[y as usize][(x + 1) as usize] {
            let new_distance = (hero_x - (x + 1)).abs() + (hero_y - y).abs();
            possible_moves.push((Direction::Right, new_distance));
        }
    
        let (current_dx, current_dy) = self.direction.as_col_row_offset();
        let (new_x, new_y) = (x + current_dx, y + current_dy);
    
        if new_x >= 0 && new_y >= 0 && new_y < obstacles.len() as i32 && new_x < obstacles[0].len() as i32 && !obstacles[new_y as usize][new_x as usize] {
            let new_distance = (hero_x - new_x).abs() + (hero_y - new_y).abs();
            if new_distance < current_distance {
                return;
            }
        }
    
        possible_moves.sort_by_key(|&(_, dist)| dist);
    
        if let Some(&(best_direction, _)) = possible_moves.first() {
            self.direction = best_direction;
        }
    }
}