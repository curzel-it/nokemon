use crate::{constants::{BASE_ENTITY_SPEED, COLLISION_THRESHOLD, SCALE}, game_engine::{collision_detection::Collision, entity::Entity, world::World}};

pub fn move_linearly(entity: &mut dyn Entity, world: &World, time_since_last_update: f32) { 
    if entity.body().is_rigid {
        let no_collisions: Vec<Collision> = vec![];
        let collisions = world.collisions.get(&entity.id()).unwrap_or(&no_collisions);
        let can_move = try_fix_collisions(entity, collisions);

        if can_move {
            just_move(entity, time_since_last_update);     
        }
    } else {
        just_move(entity, time_since_last_update); 
    }
}

fn try_fix_collisions(entity: &mut dyn Entity, collisions: &Vec<Collision>) -> bool {
    true
}

fn just_move(entity: &mut dyn Entity, time_since_last_update: f32) {
    let frame = entity.body().frame;
    let offset = entity.body().direction * entity.body().current_speed * time_since_last_update * SCALE * BASE_ENTITY_SPEED;
    let expected_x = frame.x + offset.x;
    let expected_y = frame.y + offset.y;
    entity.place_at(expected_x, expected_y);
}

#[cfg(test)]
mod tests {
    use raylib::math::{Rectangle, Vector2};

    use crate::{constants::{BASE_ENTITY_SPEED, COLLISION_BOUNCE_FIX, RECT_ORIGIN_SQUARE_100, SCALE}, game_engine::{entity::Entity, entity_body::EmbodiedEntity, simple_entity::SimpleEntity, world::World}};
    
    #[test]
    fn can_move_on_update() {
        let world = World::test();
        
        let mut body = world.entity_factory.build("red");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 1.0;        
        
        let mut entity = SimpleEntity::new(body);
        entity.body_mut().direction = Vector2::new(1.0, 0.0);  
        entity.update(&world, 1.0);

        assert_eq!(entity.body().frame.x, SCALE * BASE_ENTITY_SPEED);
        assert_eq!(entity.body().frame.y, 0.0);
    }

    #[test]
    fn can_move_outside_of_bounds() {
        let world = World::test();
        
        let mut body = world.entity_factory.build("red");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 1.0;
        
        let mut entity = SimpleEntity::new(body);
        entity.body_mut().direction = Vector2::new(-1.0, 0.0);  
        entity.update(&world, 1.0);

        assert_eq!(entity.body().frame.x, -SCALE * BASE_ENTITY_SPEED);
        assert_eq!(entity.body().frame.y, 0.0);
    }

    #[test]
    fn can_move_away_from_collision() {
        let mut world = World::test();
        world.camera_viewport = RECT_ORIGIN_SQUARE_100;
        
        let mut body1 = world.entity_factory.build("red");
        body1.id = 1001;
        body1.frame = Rectangle::new(0.0, 0.0, 10.0, 10.0);
        body1.direction = Vector2::new(-1.0, 0.0);  
        body1.requires_collision_detection = true;
        body1.current_speed = 1.0;               
        body1.is_ally = true; 
        body1.is_rigid = true;
        let entity1 = SimpleEntity::new(body1);
        world.add_entity(Box::new(entity1));
        
        let mut body2 = world.entity_factory.build("red");
        body2.id = 1002;
        body2.frame = Rectangle::new(9.0, 9.0, 10.0, 10.0);
        body2.direction = Vector2::zero();
        body2.current_speed = 1.0;
        body2.is_ally = false;
        body2.is_rigid = true;
        let entity2 = SimpleEntity::new(body2);
        world.add_entity(Box::new(entity2));
        
        world.update(1.0);

        let entities = world.entities.borrow();
        let updated_frame = entities.get(&1001).unwrap().body().frame;
        
        assert_eq!(updated_frame.x, -SCALE * BASE_ENTITY_SPEED);
        assert_eq!(updated_frame.y, -1.0);
    }

    #[test]
    fn can_not_move_in_direction_of_collisions_with_rigid_body() {
        let mut world = World::test();
        world.camera_viewport = RECT_ORIGIN_SQUARE_100;
        
        let mut body1 = world.entity_factory.build("red");
        body1.id = 1001;
        body1.frame = Rectangle::new(0.0, 0.0, 10.0, 10.0);
        body1.direction = Vector2::new(1.0, 0.0);  
        body1.requires_collision_detection = true;
        body1.current_speed = 1.0;               
        body1.is_ally = true; 
        body1.is_rigid = true;
        let entity1 = SimpleEntity::new(body1);
        world.add_entity(Box::new(entity1));
        
        let mut body2 = world.entity_factory.build("red");
        body2.id = 1002;
        body2.frame = Rectangle::new(9.0, 0.0, 10.0, 10.0);
        body2.direction = Vector2::zero();
        body2.current_speed = 1.0;
        body2.is_ally = false;
        body2.is_rigid = true;
        let entity2 = SimpleEntity::new(body2);
        world.add_entity(Box::new(entity2));
        
        world.update(1.0);

        let entities = world.entities.borrow();
        let updated_frame = entities.get(&1001).unwrap().body().frame;
        
        assert_eq!(updated_frame.x, 0.0);
        assert_eq!(updated_frame.y, 0.0);
    }

    #[test]
    fn can_move_through_small_collison() {
        let mut world = World::test();
        world.camera_viewport = RECT_ORIGIN_SQUARE_100;
        
        let mut body1 = world.entity_factory.build("red");
        body1.id = 1001;
        body1.frame = Rectangle::new(0.0, 0.0, 10.0, 10.0);
        body1.direction = Vector2::zero();  
        body1.requires_collision_detection = true;
        body1.current_speed = 1.0;               
        body1.is_ally = true; 
        body1.is_rigid = true;
        let entity1 = SimpleEntity::new(body1);
        world.add_entity(Box::new(entity1));
        
        let mut body2 = world.entity_factory.build("red");
        body2.id = 1002;
        body2.frame = Rectangle::new(10.0, 0.0, 10.0, 10.0);
        body2.direction = Vector2::zero();
        body2.current_speed = 1.0;
        body2.is_ally = false;
        body2.is_rigid = true;
        let entity2 = SimpleEntity::new(body2);
        world.add_entity(Box::new(entity2));
        
        let mut body3 = world.entity_factory.build("red");
        body3.id = 1003;
        body3.frame = Rectangle::new(0.0, 10.0, 5.0, 5.0);
        body3.direction = Vector2::new(1.0, 0.0);
        body3.current_speed = 1.0;
        body3.is_ally = false;
        body3.is_rigid = true;
        let entity3 = SimpleEntity::new(body3);
        world.add_entity(Box::new(entity3));
        
        world.update(1.0);

        let entities = world.entities.borrow();
        let updated_frame = entities.get(&1003).unwrap().body().frame;
        
        assert_eq!(updated_frame.x, SCALE * BASE_ENTITY_SPEED);
        assert_eq!(updated_frame.y, 10.0);
    }

    #[test]
    fn can_move_through_small_collison_2() {
        let mut world: World = World::test();
        world.camera_viewport = RECT_ORIGIN_SQUARE_100;
        
        let mut body1 = world.entity_factory.build("red");
        body1.id = 1001;
        body1.frame = Rectangle::new(0.0, 0.0, 10.0, 10.0);
        body1.direction = Vector2::zero();  
        body1.requires_collision_detection = true;
        body1.current_speed = 1.0;               
        body1.is_ally = true; 
        body1.is_rigid = true;
        let entity1 = SimpleEntity::new(body1);
        world.add_entity(Box::new(entity1));
        
        let mut body2 = world.entity_factory.build("red");
        body2.id = 1002;
        body2.frame = Rectangle::new(10.0, 0.0, 10.0, 10.0);
        body2.direction = Vector2::zero();
        body2.current_speed = 1.0;
        body2.is_ally = false;
        body2.is_rigid = true;
        let entity2 = SimpleEntity::new(body2);
        world.add_entity(Box::new(entity2));
        
        let mut body3 = world.entity_factory.build("red");
        body3.id = 1003;
        body3.frame = Rectangle::new(0.0, 9.0, 5.0, 5.0);
        body3.direction = Vector2::new(1.0, 0.0);
        body3.current_speed = 1.0;
        body3.is_ally = false;
        body3.is_rigid = true;
        let entity3 = SimpleEntity::new(body3);
        world.add_entity(Box::new(entity3));
        
        world.update(1.0);

        let entities = world.entities.borrow();
        let updated_frame = entities.get(&1003).unwrap().body().frame;
        
        assert_eq!(updated_frame.x, SCALE * BASE_ENTITY_SPEED);
        assert_eq!(updated_frame.y, 10.0);
    }
}