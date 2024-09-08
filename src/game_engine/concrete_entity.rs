use serde::{Deserialize, Serialize};

use crate::{constants::{SPRITE_SHEET_ANIMATED_OBJECTS, SPRITE_SHEET_BUILDINGS, SPRITE_SHEET_HOUSEHOLD_OBJECTS, SPRITE_SHEET_HUMANOIDS, SPRITE_SHEET_TELEPORTER, WORLD_ID_NONE}, dialogues::{models::{Dialogue, EntityDialogues}, repository::dialogue_by_id}, features::animated_sprite::AnimatedSprite, utils::{directions::Direction, ids::get_next_id, rect::Rect, vector::Vector2d}};

use super::{state_updates::{EngineStateUpdate, WorldStateUpdate}, storage::get_value_for_key, world::World};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Hero,
    Building(BuildingType),   
    Npc(NpcType), 
    HouseholdObject(HouseholdObject),
    PickableObject(PickableObject),
    Teleporter,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdObject {
    StairsUp,
    StairsDown,
    SeatBrown,
    SeatGreen,
    SeatOrange,
    SeatPink,
    Table,
    Bed,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum PickableObject {
    Key
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcType {
    OldMan,
    OldWoman,
}

pub type NpcId = u32;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingType {
    House(i32),
    HouseTwoFloors(i32),
}

#[derive(Debug, Copy, Clone)]
pub struct EntityProps {
    pub direction: Direction,
    pub frame: Rect,
    pub speed: f32,
    pub hittable_frame: Rect,
}

impl Default for EntityProps {
    fn default() -> Self {
        Self { 
            direction: Default::default(), 
            frame: Rect::square_from_origin(1), 
            speed: 0.0,
            hittable_frame: Rect::square_from_origin(1) 
        }
    }
}

impl EntityType {
    pub fn make_entity(&self) -> ConcreteEntity {
        let frame = self.texture_source_rect(Direction::Unknown, 0.0, false);

        ConcreteEntity {
            id: get_next_id(),
            frame,  
            species: self.clone(),  
            offset: Vector2d::zero(),
            direction: Direction::Unknown,
            current_speed: 0.0,
            is_rigid: self.is_rigid(),
            z_index: self.z_index(),
            sprite: self.make_sprite(false),
            dialogues: EntityDialogues::empty(),
            time_immobilized: 0.0,
            name: "".to_string(),
            destination: WORLD_ID_NONE,
        }
    }
}

impl EntityType {
    fn make_sprite(&self, creative_mode: bool) -> AnimatedSprite {
        let frame = self.texture_source_rect(Direction::Unknown, 0.0, creative_mode);
        AnimatedSprite::new(
            self.sprite_sheet(), 
            self.number_of_frames(), 
            frame.w, 
            frame.h
        )
    }
}

impl EntityType {
    fn z_index(&self) -> i32 {
        match self {
            EntityType::Hero => 150,
            EntityType::Npc(_) => 0,
            EntityType::Building(_) => 0,
            EntityType::HouseholdObject(item_type) => match item_type {
                HouseholdObject::StairsUp => 1000,
                HouseholdObject::StairsDown => 1000,
                HouseholdObject::SeatBrown => 100,
                HouseholdObject::SeatGreen => 100,
                HouseholdObject::SeatOrange => 100,
                HouseholdObject::SeatPink => 100,
                HouseholdObject::Table => 200,
                HouseholdObject::Bed => 200,
            }
            EntityType::PickableObject(_) => 200,
            EntityType::Teleporter => 200,
        }
        
    }
    
    pub fn base_speed(&self) -> f32 {
        match self {
            EntityType::Hero => 3.0,
            EntityType::Npc(_) => 2.0,
            EntityType::Building(_) => 2.0,
            EntityType::HouseholdObject(_) => 0.0,
            EntityType::PickableObject(_) => 0.0,
            EntityType::Teleporter => 0.0,
        }
    }

    fn is_rigid(&self) -> bool {
        match self {
            EntityType::Hero => true,
            EntityType::Building(_) => true,
            EntityType::Npc(_) => true,
            EntityType::HouseholdObject(item) => match item {
                HouseholdObject::StairsUp => true,
                HouseholdObject::StairsDown => true,
                HouseholdObject::SeatBrown => false,
                HouseholdObject::SeatGreen => false,
                HouseholdObject::SeatOrange => false,
                HouseholdObject::SeatPink => false,
                HouseholdObject::Table => true,
                HouseholdObject::Bed => true,
            },
            EntityType::PickableObject(pickable_object) => match pickable_object {
                PickableObject::Key => false,
            },
            EntityType::Teleporter => false,
        }
    }

    fn sprite_sheet(&self) -> u32 {
        match self {
            EntityType::Hero => SPRITE_SHEET_HUMANOIDS,
            EntityType::Building(_) => SPRITE_SHEET_BUILDINGS,
            EntityType::Npc(_) => SPRITE_SHEET_HUMANOIDS,
            EntityType::HouseholdObject(_) => SPRITE_SHEET_HOUSEHOLD_OBJECTS,
            EntityType::PickableObject(_) => SPRITE_SHEET_ANIMATED_OBJECTS,
            EntityType::Teleporter => SPRITE_SHEET_TELEPORTER,
        }
    }

    fn texture_source_rect(&self, direction: Direction, speed: f32, creative_mode: bool) -> Rect {
        let (x, y, w, h) = match self {
            EntityType::Hero => humanoid_texture_source_rect(12, direction, speed),
            EntityType::Building(building_type) => match building_type {
                BuildingType::House(variant) => (0, 5 * variant + 1, 5, 4),
                BuildingType::HouseTwoFloors(variant) => (5, 5 * variant, 5, 5),
            },
            EntityType::Npc(npc_type) => match npc_type {
                NpcType::OldMan => humanoid_texture_source_rect(4, direction, speed),
                NpcType::OldWoman => humanoid_texture_source_rect(8, direction, speed),
            },
            EntityType::HouseholdObject(item) => match item {
                HouseholdObject::StairsUp => (1, 0, 1, 2),
                HouseholdObject::StairsDown => (2, 0, 1, 2),
                HouseholdObject::SeatBrown => (3, 0, 1, 1),
                HouseholdObject::SeatGreen => (3, 1, 1, 1),
                HouseholdObject::SeatOrange => (3, 2, 1, 1),
                HouseholdObject::SeatPink => (3, 3, 1, 1),
                HouseholdObject::Table => (4, 0, 2, 2),
                HouseholdObject::Bed => (0, 2, 1, 2),
            },
            EntityType::PickableObject(pickable_object) => match pickable_object {
                PickableObject::Key => (0, 0, 1, 1),
            },
            EntityType::Teleporter => (0, if creative_mode { 0 } else { 1 }, 1, 1),
        };
        Rect::new(x, y, w, h)
    }

    fn number_of_frames(&self) -> i32 {
        match self {
            EntityType::Hero => 4,
            EntityType::Building(_) => 1,
            EntityType::Npc(_) => 4,
            EntityType::HouseholdObject(_) => 1,
            EntityType::PickableObject(pickable_object) => match pickable_object {
                PickableObject::Key => 8,
            },
            EntityType::Teleporter => 1,
        }
    }
}

fn humanoid_texture_source_rect(column: i32, direction: Direction, speed: f32) -> (i32, i32, i32, i32) {
    let row = match (direction, speed != 0.0) {
        (Direction::Up, true) => 0,
        (Direction::Up, false) => 1,
        (Direction::Right, true) => 2,
        (Direction::Right, false) => 3,
        (Direction::Down, true) => 4,
        (Direction::Down, false) => 5,
        (Direction::Left, true) => 6,
        (Direction::Left, false) => 7,
        (Direction::Unknown, true) => 5,
        (Direction::Unknown, false) => 5
    };
    (column, row, 1, 2)
}

impl EntityType {
    pub fn inventory_texture_source_rect(&self) -> Rect {
        let (row, col) = self.inventory_texture_offsets();
        Rect::new(col, row, 1, 1)
    }

    pub fn inventory_texture_offsets(&self) -> (i32, i32) {
        match self {
            EntityType::Hero => (0, 0),
            EntityType::Building(building_type) => match building_type {
                BuildingType::House(variant) => (4, variant * 2 + 1),
                BuildingType::HouseTwoFloors(variant) => (4, variant * 2 + 2),
            },
            EntityType::Npc(npc_type) => match npc_type {
                NpcType::OldMan => (2, 2),
                NpcType::OldWoman => (2, 3),
            },
            EntityType::HouseholdObject(item) => match item {
                HouseholdObject::StairsUp => (3, 2),
                HouseholdObject::StairsDown => (3, 3),
                HouseholdObject::SeatBrown => (3, 4),
                HouseholdObject::SeatGreen => (3, 5),
                HouseholdObject::SeatOrange => (3, 6),
                HouseholdObject::SeatPink => (3, 7),
                HouseholdObject::Table => (3, 8),
                HouseholdObject::Bed => (3, 9),
            },
            EntityType::PickableObject(pickable_object) => match pickable_object {
                PickableObject::Key => (5, 1),
            },
            EntityType::Teleporter => (0, 0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConcreteEntity {
    pub id: u32,
    pub frame: Rect,  
    pub name: String,  
    pub species: EntityType,  
    pub offset: Vector2d,
    pub direction: Direction,
    pub current_speed: f32,
    pub is_rigid: bool,
    pub z_index: i32,
    pub sprite: AnimatedSprite,
    pub dialogues: EntityDialogues,
    pub time_immobilized: f32,
    pub destination: u32,
}

impl ConcreteEntity {
    pub fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {      
        let updates = match self.species {
            EntityType::Hero => self.update_hero(world, time_since_last_update),
            EntityType::Npc(_) => self.update_npc(world, time_since_last_update),
            EntityType::Building(_) => self.update_generic(world, time_since_last_update),
            EntityType::HouseholdObject(_) => self.update_generic(world, time_since_last_update),
            EntityType::PickableObject(_) => self.update_pickable_object(world, time_since_last_update),
            EntityType::Teleporter => self.update_teleporter(world, time_since_last_update),
        };
        
        self.time_immobilized -= time_since_last_update;
        if self.time_immobilized <= 0.0 {
            self.move_linearly(world, time_since_last_update)
        }
        
        self.sprite.update(time_since_last_update);  

        updates
    }

    pub fn sprite_sheet(&self) -> u32 {
        self.species.sprite_sheet()
    }

    pub fn texture_source_rect(&self, creative_mode: bool) -> Rect {
        self.species.texture_source_rect(self.direction, self.current_speed, creative_mode)
    }

    pub fn immobilize_for_seconds(&mut self, seconds: f32) {
        self.time_immobilized = seconds;
    }

    pub fn reset_speed(&mut self) {
        self.current_speed = self.species.base_speed();
    }    
    
    pub fn center_in(&mut self, value: &Rect) {
        self.frame.center_in(value)
    }

    fn next_dialogue(&self) -> Option<Dialogue> {
        for option in &self.dialogues.options {
            if let Some(value) = get_value_for_key(&option.key) {
                if value == option.expected_value {
                    return dialogue_by_id(option.dialogue)
                }
            }
        }
        None
    }
}

impl ConcreteEntity {
    fn update_hero(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {        
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        
        self.set_direction_based_on_current_keys(world.direction_based_on_current_keys);
        
        world_updates.push(self.cache_props());
        world_updates.push(self.move_camera_update());
        world_updates
    }

    fn cache_props(&self) -> WorldStateUpdate {
        WorldStateUpdate::CacheHeroProps(
            self.props()           
        )
    }

    fn props(&self) -> EntityProps {
        EntityProps {
            frame: self.frame,
            direction: self.direction,
            speed: self.current_speed,
            hittable_frame: Rect {
                x: self.frame.x,
                y: self.frame.y + 1,
                w: 1,
                h: 1,
            }
        }            
    }

    fn move_camera_update(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::CenterCamera(
                self.frame.x, 
                self.frame.y,
                self.offset
            )
        )
    }
}

impl ConcreteEntity {
    fn update_npc(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {  
        if world.is_hero_around_and_on_collision_with(&self.frame) {
            if world.creative_mode {
                return vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowNpcOptions(
                            self.id, self.name.clone(), self.next_dialogue()
                        )
                    )
                ];  
            } else if let Some(dialogue) = self.next_dialogue() {
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
}

impl ConcreteEntity {
    fn update_generic(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {  
        if world.creative_mode && world.is_hero_around_and_on_collision_with(&self.frame) {
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        self.id
                    )
                )
            ];   
        }
        vec![]
    }
}

impl ConcreteEntity {
    fn update_pickable_object(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {        
        vec![]
    }
}

impl ConcreteEntity {
    fn update_teleporter(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {      
        if self.should_teleport(world) {
            vec![self.engine_update_push_world()]
        } else {
            vec![]
        }        
    }
    fn should_teleport(&self, world: &World) -> bool {
        let hero = world.cached_hero_props.hittable_frame;
        let hero_direction = world.cached_hero_props.direction;
        let hero_speed = world.cached_hero_props.speed;

        if !world.is_any_arrow_key_down { return false }
        if hero_speed <= 0.0 { return false }

        match hero_direction {
            Direction::Up => hero.x == self.frame.x && hero.y == self.frame.y + 1,
            Direction::Right => hero.y == self.frame.y && hero.x == self.frame.x - 1,
            Direction::Down => hero.x == self.frame.x && hero.y == self.frame.y - 1,
            Direction::Left => hero.y == self.frame.y && hero.x == self.frame.x + 1,
            Direction::Unknown => false
        }
    }

    fn engine_update_push_world(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::SwitchWorld(self.destination)
        )
    }
}