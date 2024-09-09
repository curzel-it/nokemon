use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{constants::{ANIMATIONS_FPS, SPRITE_SHEET_ANIMATED_OBJECTS, SPRITE_SHEET_BUILDINGS, SPRITE_SHEET_HOUSEHOLD_OBJECTS, SPRITE_SHEET_HUMANOIDS, SPRITE_SHEET_TELEPORTER}, entities::{buildings::BuildingType, household_objects::HouseholdObject, npcs::NpcType, pickable_objects::PickableObject, species::Species}, game_engine::entity::Entity, utils::{directions::Direction, rect::Rect, timed_content_provider::TimedContentProvider}};

#[derive(Debug)]
pub struct AnimatedSprite {
    sheet_id: u32, 
    pub frame: Rect,
    original_frame: Rect,
    number_of_frames: i32,
    frames_provider: TimedContentProvider<i32>,
}

impl AnimatedSprite {
    pub fn new(sheet_id: u32, frame: Rect, number_of_frames: i32) -> Self {
        Self {
            sheet_id, 
            frame,
            original_frame: frame,
            number_of_frames,
            frames_provider: TimedContentProvider::frames(frame.x, number_of_frames, frame.w),
        }
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        self.frames_provider.update(time_since_last_update);
        self.frame.x = *self.frames_provider.current_frame();
    }

    pub fn texture_source_rect(&self) -> Rect {
        self.frame
    }
}

impl Entity {
    pub fn update_sprite_for_current_direction(&mut self) {
        self.sprite.frame.y = match (self.direction, self.current_speed != 0.0) {
            (Direction::Up, true) => 0,
            (Direction::Up, false) => 2,
            (Direction::Right, true) => 4,
            (Direction::Right, false) => 6,
            (Direction::Down, true) => 8,
            (Direction::Down, false) => 10,
            (Direction::Left, true) => 12,
            (Direction::Left, false) => 14,
            (Direction::Unknown, true) => 10,
            (Direction::Unknown, false) => 10
        }
    }
}

impl Species {
    pub fn make_sprite(&self, creative_mode: bool) -> AnimatedSprite {
        AnimatedSprite::new(
            self.sprite_sheet(), 
            self.texture_source_rect(creative_mode), 
            self.number_of_frames(), 
        )
    }

    pub fn sprite_sheet(&self) -> u32 {
        match self {
            Species::Hero => SPRITE_SHEET_HUMANOIDS,
            Species::Building(_) => SPRITE_SHEET_BUILDINGS,
            Species::Npc(_) => SPRITE_SHEET_HUMANOIDS,
            Species::HouseholdObject(_) => SPRITE_SHEET_HOUSEHOLD_OBJECTS,
            Species::PickableObject(_) => SPRITE_SHEET_ANIMATED_OBJECTS,
            Species::Teleporter => SPRITE_SHEET_TELEPORTER,
        }
    }

    pub fn texture_source_rect(&self, creative_mode: bool) -> Rect {
        let (x, y, w, h) = match self {
            Species::Hero => (12, 0, 1, 2),
            Species::Building(building_type) => match building_type {
                BuildingType::House(variant) => (0, 5 * variant + 1, 5, 4),
                BuildingType::HouseTwoFloors(variant) => (5, 5 * variant, 5, 5),
            },
            Species::Npc(npc_type) => match npc_type {
                NpcType::OldMan => (4, 0, 1, 2),
                NpcType::OldWoman => (8, 0, 1, 2),
            },
            Species::HouseholdObject(item) => match item {
                HouseholdObject::StairsUp => (1, 0, 1, 2),
                HouseholdObject::StairsDown => (2, 0, 1, 2),
                HouseholdObject::SeatBrown => (3, 0, 1, 1),
                HouseholdObject::SeatGreen => (3, 1, 1, 1),
                HouseholdObject::SeatOrange => (3, 2, 1, 1),
                HouseholdObject::SeatPink => (3, 3, 1, 1),
                HouseholdObject::Table => (4, 0, 2, 2),
                HouseholdObject::Bed => (0, 2, 1, 2),
            },
            Species::PickableObject(pickable_object) => match pickable_object {
                PickableObject::Key => (0, 0, 1, 1),
            },
            Species::Teleporter => (0, if creative_mode { 0 } else { 1 }, 1, 1),
        };
        Rect::new(x, y, w, h)
    }

    fn number_of_frames(&self) -> i32 {
        match self {
            Species::Hero => 4,
            Species::Building(_) => 1,
            Species::Npc(_) => 4,
            Species::HouseholdObject(_) => 1,
            Species::PickableObject(pickable_object) => match pickable_object {
                PickableObject::Key => 8,
            },
            Species::Teleporter => 1,
        }
    }
}

impl TimedContentProvider<i32> {
    pub fn frames(x: i32, n: i32, w: i32) -> Self {
        let frames = (0..n).map(|i| x + i as i32 * w).collect();
        Self::new(frames, ANIMATIONS_FPS)
    }
}

#[derive(Serialize, Deserialize)]
struct AnimatedSpriteData {
    sheet_id: u32, 
    frame: Rect,
    number_of_frames: i32,
}

impl Serialize for AnimatedSprite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let data = AnimatedSpriteData {
            sheet_id: self.sheet_id, 
            frame: self.original_frame, 
            number_of_frames: self.number_of_frames,
        };
        data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AnimatedSprite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let AnimatedSpriteData { sheet_id, frame, number_of_frames } = AnimatedSpriteData::deserialize(deserializer)?;
        let sprite = AnimatedSprite::new(sheet_id, frame, number_of_frames);
        Ok(sprite)
    }
}
