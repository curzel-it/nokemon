use serde::{Deserialize, Serialize};

use crate::{game_engine::entity::Entity, utils::{directions::Direction, vector::Vector2d}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patrol {
    pub initial_position: (i32, i32),
    pub current_index: usize,
    pub movements: Vec<PatrolMovement>
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PatrolMovement {
    pub direction: Direction,
    pub steps: i32,
    pub steps_left: i32,
}

impl Patrol {
    pub fn none() -> Self {
        Self::new(0, 0, vec![])
    }

    pub fn new(x: i32, y: i32, movements: Vec<PatrolMovement>) -> Self {
        Self {
            initial_position: (x, y),
            current_index: 0,
            movements
        }
    }
}

impl Default for Patrol {
    fn default() -> Self {
        Self::none()
    }
}

impl Entity {
    pub fn handle_patrol(&mut self) {
        let new_steps = self.latest_movement.0.abs() + self.latest_movement.1.abs();

        if self.patrol.movements.is_empty() { 
            return 
        }
        self.reset_speed();
        self.direction = self.patrol.update(new_steps);
    }

    pub fn setup_patrol(&mut self) {
        if self.patrol.movements.is_empty() { 
            return 
        }
        (self.frame.x, self.frame.y) = self.patrol.initial_position;
        self.offset = Vector2d::zero();
        self.patrol.current_index = 0;

        for movement in &mut self.patrol.movements {
            movement.steps_left = movement.steps;
        }

    }
}

impl Patrol {
    fn update(&mut self, new_steps: i32) -> Direction {
        let current = &mut self.movements[self.current_index];

        if new_steps > current.steps_left {
            current.steps_left = current.steps;

            if self.current_index == self.movements.len() - 1 {
                self.current_index = 0;
            } else {
                self.current_index += 1;
            }
        } else {
            current.steps_left -= new_steps;
        }
        return self.movements[self.current_index].direction;
    }
}