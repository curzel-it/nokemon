
use crate::{constants::{BASE_ENTITY_SPEED, TILE_SIZE}, game_engine::{collision_detection::Collision, entity::Entity, world::World}, utils::vector::Vector2d};

/*
1. Compute a hitmap via WorldStateUpdates... rigid_bodies_map and collidables_map ?
2. Check rows and cols
3. Update offset if possible
3. If offset > TILE_SIZE -> move to next tile
*/

pub fn move_linearly(entity: &mut dyn Entity, world: &World, time_since_last_update: f32) { 
    let no_collisions: Vec<Collision> = vec![];
    let collisions = world.collisions.get(&entity.id()).unwrap_or(&no_collisions);
    
    if has_blocking_collisions(entity, collisions) {
        return
    }
    
    let updated_offset = updated_offset(entity, time_since_last_update);    
    let tiles_x_f = updated_offset.x / TILE_SIZE;
    let tiles_y_f = updated_offset.y / TILE_SIZE;
    let tiles_x = if updated_offset.x > 0.0 { tiles_x_f.floor() } else { tiles_x_f.ceil() };
    let tiles_y = if updated_offset.y > 0.0 { tiles_y_f.floor() } else { tiles_y_f.ceil() };

    entity.body_mut().frame = entity.body().frame.offset(
        tiles_x as i32, 
        tiles_y as i32
    );
    entity.body_mut().offset = Vector2d::new(
        updated_offset.x - tiles_x * TILE_SIZE,
        updated_offset.y - tiles_y * TILE_SIZE
    );
}

fn updated_offset(entity: &dyn Entity, time_since_last_update: f32) -> Vector2d {
    entity.body().direction
        .scaled(entity.body().current_speed)
        .scaled(time_since_last_update)
        .scaled(BASE_ENTITY_SPEED) + entity.body().offset
}

fn has_blocking_collisions(entity: &dyn Entity, collisions: &Vec<Collision>) -> bool {
    if entity.body().is_rigid && entity.body().requires_collision_detection {
        let rigid_collisions: Vec<&Collision> = collisions.iter().filter(|c| c.other_was_rigid).collect();
        return has_blocking_rigid_collisions(entity, &rigid_collisions);        
    }
    false
}

fn has_blocking_rigid_collisions(entity: &dyn Entity, collisions: &Vec<&Collision>) -> bool {
    let entity_center_x = entity.body().frame.x + entity.body().frame.w / 2;
    let entity_center_y = entity.body().frame.y + entity.body().frame.h / 2;
    let direction = entity.body().direction;

    if direction.x > 0.0 {
        return collisions.iter().any(|collision| {
            collision.center_x > entity_center_x
        });
    }
    if direction.x < 0.0 {
        return collisions.iter().any(|collision| {
            collision.center_x < entity_center_x
        });
    }
    if direction.y > 0.0 {
        return collisions.iter().any(|collision| {
            collision.center_y > entity_center_y
        });
    }
    if direction.y < 0.0 {
        return collisions.iter().any(|collision| {
            collision.center_y < entity_center_y
        });
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::{constants::{BASE_ENTITY_SPEED, TILE_SIZE}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, simple_entity::SimpleEntity, world::World}, utils::{rect::Rect, vector::Vector2d}, worlds::constants::WORLD_ID_DEMO};
    
    #[test]
    fn can_move_right_on_update() {
        let world = World::new(WORLD_ID_DEMO);
        
        let mut body = EntityBody::test();
        body.frame = Rect::square_from_origin(100);
        body.current_speed = 1.0;        
        
        let mut entity = SimpleEntity::new(body);
        entity.body_mut().direction = Vector2d::new(1.0, 0.0);  
        entity.update(&world, 1.0);

        let expected_x = (BASE_ENTITY_SPEED / TILE_SIZE).floor() as u32;
        let expected_offset = BASE_ENTITY_SPEED - TILE_SIZE * expected_x as f32;

        assert_eq!(entity.body().frame.x, expected_x);
        assert_eq!(entity.body().offset.x, expected_offset);
        assert_eq!(entity.body().frame.y, 0);
    }

    #[test]
    fn can_move_left_on_update() {
        let world = World::new(WORLD_ID_DEMO);
        
        let mut body = EntityBody::test();
        body.frame = Rect::square_from_origin(100);
        body.current_speed = 1.0;
        
        let mut entity = SimpleEntity::new(body);
        entity.body_mut().frame.x = 50;
        entity.body_mut().direction = Vector2d::new(-1.0, 0.0);  
        entity.update(&world, 1.0);

        let speed_tiles = (BASE_ENTITY_SPEED / TILE_SIZE).floor() as u32;
        let expected_x = 50 - speed_tiles;
        let expected_offset = TILE_SIZE * speed_tiles as f32 - BASE_ENTITY_SPEED;

        assert_eq!(entity.body().frame.x, expected_x);
        assert_eq!(entity.body().offset.x, expected_offset);
        assert_eq!(entity.body().frame.y, 0);
    }

    #[test]
    fn can_not_move_outside_of_bounds() {
        let world = World::new(WORLD_ID_DEMO);
        
        let mut body = EntityBody::test();
        body.frame = Rect::square_from_origin(100);
        body.current_speed = 1.0;
        
        let mut entity = SimpleEntity::new(body);
        entity.body_mut().direction = Vector2d::new(-1.0, 0.0);  
        entity.update(&world, 1.0);

        assert_eq!(entity.body().frame.x, 0);
        assert_eq!(entity.body().frame.y, 0);
    }
}