

use raylib::math::Rectangle;

use crate::{constants::{ASSETS_PATH, TILE_SIZE, TILE_TEXTURE_SIZE}, impl_tile, utils::geometry_utils::Direction};

use super::tiles::SpriteTile;

pub const COLOR_GRASS: u32 = 0x00FF00;
pub const COLOR_WATER: u32 = 0x0000FF;
pub const COLOR_ROCK: u32 = 0x7F7F7F;
pub const COLOR_DESERT: u32 = 0xFFFF00;
pub const COLOR_SNOW: u32 = 0xFFFFFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Biome {
    Grass,
    Water,
    Rock,
    Desert, 
    Snow
}

#[derive(Debug, Clone)]
pub struct BiomeTile {
    pub tile_type: Biome,
    pub column: u32, 
    pub row: u32,
    pub width: u32,
    pub height: u32,
    pub tile_up_type: Biome,
    pub tile_right_type: Biome,
    pub tile_down_type: Biome,
    pub tile_left_type: Biome,
    pub texture_offset_x: f32,
    pub texture_offset_y: f32,
}

impl Default for BiomeTile {
    fn default() -> Self {
        BiomeTile {
            tile_type: Biome::Grass,
            column: 0,
            row: 0,
            width: 1,
            height: 1,
            tile_up_type: Biome::Grass,
            tile_right_type: Biome::Grass,
            tile_down_type: Biome::Grass,
            tile_left_type: Biome::Grass,
            texture_offset_x: 0.0,
            texture_offset_y: 0.0,
        }
    }
}

impl_tile!(BiomeTile);

impl SpriteTile for BiomeTile {
    fn sprite_name(&self) -> String {
        format!("{}/bg_tiles.png", ASSETS_PATH)
    }

    fn sprite_source_rect(&self, variant: u32) -> Rectangle {
        Rectangle::new(
            self.texture_offset_x,
            self.texture_offset_y + TILE_TEXTURE_SIZE * (variant * Biome::number_of_biomes()) as f32,
            TILE_TEXTURE_SIZE, 
            TILE_TEXTURE_SIZE
        )
    }
}

impl BiomeTile {
    pub fn setup_neighbors(&mut self, up: Biome, right: Biome, bottom: Biome, left: Biome) {
        self.tile_up_type = up;
        self.tile_right_type = right;
        self.tile_down_type = bottom;
        self.tile_left_type = left;        
        self.setup_mixed_biomes();    
    }

    fn setup_mixed_biomes(&mut self) {
        let x = self.texture_index_for_neighbors();
        let y = self.tile_type.texture_index(); 
        self.texture_offset_x = TILE_TEXTURE_SIZE * x as f32;
        self.texture_offset_y = TILE_TEXTURE_SIZE * y as f32;
    }

    fn texture_index_for_neighbors(&self) -> u32 {
        if let Some((neighbor, directions)) = self.best_neighbor() {            
            return match (self.tile_type, neighbor) {
                (Biome::Water, Biome::Desert) => 0,
                (Biome::Grass, Biome::Desert) => 0,
                (Biome::Grass, Biome::Rock) => 0,
                (Biome::Snow, Biome::Rock) => 0,
                _ => neighbor.texture_index() * Biome::number_of_combinations() + self.texture_index_for_directions(&directions) + 1
            }
        }        
        0 
    }

    fn texture_index_for_directions(&self, directions: &Vec<Direction>) -> u32 {
        if directions.len() == 1 {
            match directions[0] {
                Direction::Up => return 0,
                Direction::Right => return 1,
                Direction::Down => return 2,
                Direction::Left => return 3,
            }
        }
        if directions.len() == 2 {
            match (directions[0], directions[1]) {
                (Direction::Up, Direction::Left) => return 4,
                (Direction::Up, Direction::Right) => return 5,
                (Direction::Right, Direction::Down) => return 6,
                (Direction::Down, Direction::Left) => return 7,
                _ => {}
            }
        }
        if directions.len() == 3 {
            match (directions[0], directions[1], directions[2]) {
                (Direction::Up, Direction::Right, Direction::Down) => return 8,
                (Direction::Right, Direction::Down, Direction::Left) => return 9,
                (Direction::Up, Direction::Down, Direction::Left) => return 10,
                (Direction::Up, Direction::Right, Direction::Left) => return 11,
                _ => {}
            }
        }
        if directions.len() == 4 {
            return 12;
        }
        0
    }

    fn best_neighbor(&self) -> Option<(Biome, Vec<Direction>)> {
        let up = self.contact_directions_with_biome(self.tile_up_type);
        let right = self.contact_directions_with_biome(self.tile_right_type);
        let down = self.contact_directions_with_biome(self.tile_down_type);
        let left = self.contact_directions_with_biome(self.tile_left_type);

        let upc = up.len();
        let rightc = right.len();
        let downc = down.len();
        let leftc = left.len();

        for i in 1..=3 {
            if self.tile_up_type != self.tile_type && upc >= 3-i {
                return Some((self.tile_up_type, up));
            }
            if self.tile_right_type != self.tile_type && rightc >= 3-i {
                return Some((self.tile_right_type, right));
            }
            if self.tile_down_type != self.tile_type && downc >= 3-i {
                return Some((self.tile_down_type, down));
            }
            if self.tile_left_type != self.tile_type && leftc >= 3-i {
                return Some((self.tile_left_type, left));
            }
        }
        None
    }

    fn contact_directions_with_biome(&self, biome: Biome) -> Vec<Direction> {
        let mut contacts: Vec<Direction> = vec![];
        if self.tile_up_type == biome { contacts.push(Direction::Up); }
        if self.tile_right_type == biome { contacts.push(Direction::Right); }
        if self.tile_down_type == biome { contacts.push(Direction::Down); }
        if self.tile_left_type == biome { contacts.push(Direction::Left); }
        contacts
    }

}

impl Biome {
    fn number_of_combinations() -> u32 {
        13
    }

    fn number_of_biomes() -> u32 {
        5
    }

    fn animation_name(&self) -> &str {
        match self {
            Biome::Water => "water",
            Biome::Desert => "desert",
            Biome::Grass => "grass",
            Biome::Rock => "rock",
            Biome::Snow => "snow",
        }
    }

    fn texture_index(&self) -> u32 {
        match self {
            Biome::Water => 0,
            Biome::Desert => 1,
            Biome::Grass => 2,
            Biome::Rock => 3,
            Biome::Snow => 4,
        }
    }
    
    fn from_color(color: u32) -> Option<Biome> {
        match color {
            COLOR_GRASS => Some(Biome::Grass),
            COLOR_WATER => Some(Biome::Water),
            COLOR_ROCK => Some(Biome::Rock),
            COLOR_DESERT => Some(Biome::Desert),
            COLOR_SNOW => Some(Biome::Snow),
            _ => None,
        }
    }
}

impl BiomeTile {
    pub fn with_color_indeces(color: u32, column: u32, row: u32) -> Self {
        Self::with_color_indeces_size(color, column, row, 1, 1)
    }

    pub fn with_color_indeces_size(color: u32, column: u32, row: u32, width: u32, height: u32) -> Self {
        let tile_type = Biome::from_color(color).unwrap_or(Biome::Desert);            
        
        Self {
            tile_type,
            column, 
            row,
            width,
            height,
            tile_up_type: Biome::Grass,
            tile_right_type: Biome::Grass,
            tile_down_type: Biome::Grass,
            tile_left_type: Biome::Grass,
            texture_offset_x: 0.0,
            texture_offset_y: 0.0,
        }
    }

    pub fn is_water(&self) -> bool {
        match &self.tile_type {
            Biome::Water => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::geometry_utils::Direction;

    use super::{Biome, BiomeTile, COLOR_WATER};
        
    #[test]
    fn can_return_correct_index_from_directions() {
        let tile = BiomeTile::with_color_indeces(COLOR_WATER, 0, 0);
        assert_eq!(tile.texture_index_for_directions(&vec![Direction::Up]), 0);
        assert_eq!(tile.texture_index_for_directions(&vec![Direction::Right]), 1);
        assert_eq!(tile.texture_index_for_directions(&vec![Direction::Down]), 2);
        assert_eq!(tile.texture_index_for_directions(&vec![Direction::Left]), 3);
        assert_eq!(tile.texture_index_for_directions(&vec![Direction::Up, Direction::Left]), 4);
        assert_eq!(tile.texture_index_for_directions(&vec![Direction::Up, Direction::Right]), 5);
        assert_eq!(tile.texture_index_for_directions(&vec![Direction::Right, Direction::Down]), 6);
        assert_eq!(tile.texture_index_for_directions(&vec![Direction::Down, Direction::Left]), 7);
        assert_eq!(tile.texture_index_for_directions(&vec![Direction::Up, Direction::Right, Direction::Down]), 8);
        assert_eq!(tile.texture_index_for_directions(&vec![Direction::Right, Direction::Down, Direction::Left]), 9);
        assert_eq!(tile.texture_index_for_directions(&vec![Direction::Up, Direction::Down, Direction::Left]), 10);
        assert_eq!(tile.texture_index_for_directions(&vec![Direction::Up, Direction::Right, Direction::Left]), 11);
    }

    #[test]
    fn can_pick_up_neighbor_when_all_neighbors_are_different() {
        let mut tile = BiomeTile::with_color_indeces(COLOR_WATER, 0, 0);
        tile.setup_neighbors(Biome::Rock, Biome::Desert, Biome::Grass, Biome::Snow);

        let neighbor: Option<(Biome, Vec<Direction>)> = tile.best_neighbor();
        assert!(neighbor.is_some());
        
        let (neighbor, directions) = neighbor.unwrap();
        assert_eq!(neighbor, Biome::Rock);
        assert_eq!(directions, vec![Direction::Up]);
    }
    
    #[test]
    fn can_pick_best_neighbor_when_majority() {
        let mut tile = BiomeTile::with_color_indeces(COLOR_WATER, 0, 0);
        tile.setup_neighbors(Biome::Rock, Biome::Rock, Biome::Grass, Biome::Snow);

        let neighbor = tile.best_neighbor();
        assert!(neighbor.is_some());
        
        let (neighbor, directions) = neighbor.unwrap();
        assert_eq!(neighbor, Biome::Rock);
        assert_eq!(directions, vec![Direction::Up, Direction::Right]);
    }
    
    #[test]
    fn can_pick_best_neighbor_when_minority_if_other_is_current_tile() {
        let mut tile = BiomeTile::with_color_indeces(COLOR_WATER, 0, 0);
        tile.setup_neighbors(Biome::Water, Biome::Water, Biome::Rock, Biome::Water);

        let neighbor = tile.best_neighbor();
        assert!(neighbor.is_some());
        
        let (neighbor, directions) = neighbor.unwrap();
        assert_eq!(neighbor, Biome::Rock);
        assert_eq!(directions, vec![Direction::Down]);
    }
    
    #[test]
    fn does_not_pick_a_best_neighbor_if_all_are_the_same_as_the_current_tile() {
        let mut tile = BiomeTile::with_color_indeces(COLOR_WATER, 0, 0);
        tile.setup_neighbors(Biome::Water, Biome::Water, Biome::Water, Biome::Water);

        let neighbor = tile.best_neighbor();
        assert!(neighbor.is_none());
    }
}