use serde::Deserialize;
use std::string::String;

pub const INFINITE_LIFESPAN: f32 = -420.0;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
pub enum SpeciesCapability {
    AnimatedSprite,
    LinearMovement,
    StaysInsideScreenBounds,
    HeroAttachment,
    Bullet,
    Shooter,
    HeroSeeker
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Species {
    pub id: String,

    #[serde(default="df_zero")]
    pub speed: f32,

    #[serde(default="df_one")]
    pub width: f32,

    #[serde(default="df_one")]
    pub height: f32,

    #[serde(default="df_one_hundred")]
    pub hp: f32,

    #[serde(default="df_one_hundred")]
    pub dp: f32,

    #[serde(default="df_u32_one")]
    pub z_index: u32,

    #[serde(default="df_no_capabilities")]
    pub capabilities: Vec<SpeciesCapability>,

    #[serde(default="df_false")]
    pub is_enemy: bool,

    #[serde(default="df_one")]
    pub time_between_shots: f32,

    #[serde(default="infinite_lifespan")]
    pub lifespan: f32,
}

impl Species {
    pub fn is_bullet(&self) -> bool {
        self.capabilities.contains(&SpeciesCapability::Bullet)
    }

    pub fn is_hero_seeker(&self) -> bool {
        self.capabilities.contains(&SpeciesCapability::HeroSeeker)
    }

    pub fn is_hero_attachment(&self) -> bool {
        self.capabilities.contains(&SpeciesCapability::HeroAttachment)
    }

    pub fn is_shooter(&self) -> bool {
        self.capabilities.contains(&SpeciesCapability::Shooter)
    }

    pub fn stays_inside_screen_bounds(&self) -> bool {
        self.capabilities.contains(&SpeciesCapability::StaysInsideScreenBounds)
    }
}

fn df_no_capabilities() -> Vec<SpeciesCapability> {
    vec![
        SpeciesCapability::AnimatedSprite,
        SpeciesCapability::LinearMovement,
        SpeciesCapability::StaysInsideScreenBounds
    ]
}

fn infinite_lifespan() -> f32 {
    INFINITE_LIFESPAN
}

fn df_one() -> f32 {
    1.0
}

fn df_one_hundred() -> f32 {
    1.0
}

fn df_u32_one() -> u32 {
    1
}

fn df_zero() -> f32 {
    0.0
}

fn df_false() -> bool {
    false
}