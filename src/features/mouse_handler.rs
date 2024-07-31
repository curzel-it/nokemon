use raylib::math::Vector2;

use crate::game_engine::game::Game;

pub struct MouseHandler {
    pub dragging_id: Option<u32>,
    mouse_down: Vector2,
    pub drag_offset: Vector2,
    reset_dragging_id: bool
}

impl MouseHandler {
    pub fn new() -> Self {
        Self {
            dragging_id: None,
            mouse_down: Vector2::zero(),
            drag_offset: Vector2::zero(),
            reset_dragging_id: false
        }
    }

    pub fn handle_mouse_event(
        &mut self, 
        game: &mut Game,
        position: Vector2, 
        is_pressed: bool, 
        is_released: bool        
    ) {
        self.drag_offset = Vector2::new(
            position.x - self.mouse_down.x, 
            position.y - self.mouse_down.y
        );
        if self.reset_dragging_id {
            self.reset_dragging_id = false;
            self.dragging_id = None;
        }

        if is_pressed {
            if self.dragging_id.is_none() {
                let pointed_item = self.find_item_id_by_position(game, &position);
                self.dragging_id = pointed_item;
                self.mouse_down = position;
            }
        }

        if is_released {
            if let Some(id) = self.dragging_id {
                game.move_entity_by(id, self.drag_offset);
            }
            self.reset_dragging_id = true;
        }
    }

    fn find_item_id_by_position(&self, game: &Game, position: &Vector2) -> Option<u32> {
        for entity in game.entities.values() {
            if entity.frame.check_collision_point_rec(position) {
                return Some(entity.id);
            }
        }
        return None;
    }
}