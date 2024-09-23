use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer, de::Deserializer};

use crate::{impl_tile, utils::{directions::Direction, rect::Rect}};

use super::tiles::{SpriteTile, TileSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Biome {
    Nothing,
    Grass,
    Water,
    Rock,
    Desert, 
    Snow, 
    DarkWood, 
    LightWood,
    DarkRock,
    Ice,
    DarkGrass,
    RockPlates,
}

impl Default for Biome {
    fn default() -> Self {
        Biome::Grass
    }
}

#[derive(Default, Debug, Clone)]
pub struct BiomeTile {
    pub tile_type: Biome,
    pub column: u32, 
    pub row: u32,
    pub tile_up_type: Biome,
    pub tile_right_type: Biome,
    pub tile_down_type: Biome,
    pub tile_left_type: Biome,
    pub texture_offset_x: i32,
    pub texture_offset_y: i32,
}

impl_tile!(BiomeTile);

impl SpriteTile for BiomeTile {
    fn texture_source_rect(&self, variant: i32) -> Rect {
        Rect::new(
            self.texture_offset_x,
            self.texture_offset_y + variant * Biome::number_of_biomes(),
            1, 
            1
        )
    }
}

impl BiomeTile {
    pub fn is_obstacle(&self) -> bool {
        matches!(&self.tile_type, Biome::Water) || matches!(&self.tile_type, Biome::Nothing)
    }

    pub fn setup_neighbors(&mut self, up: Biome, right: Biome, bottom: Biome, left: Biome) {
        self.tile_up_type = up;
        self.tile_right_type = right;
        self.tile_down_type = bottom;
        self.tile_left_type = left;        
        self.setup_textures();    
    }

    fn setup_textures(&mut self) {
        self.texture_offset_x = self.texture_index_for_neighbors();
        self.texture_offset_y = self.tile_type.texture_index(); 
    }

    fn texture_index_for_neighbors(&self) -> i32 {
        if let Some((neighbor, directions)) = self.best_neighbor() {            
            return match (self.tile_type, neighbor) {
                (Biome::Water, Biome::Desert) => 0,
                (Biome::Water, Biome::Grass) => 0,
                (Biome::Water, Biome::DarkGrass) => 0,
                (Biome::Grass, Biome::Desert) => 0,
                (Biome::Grass, Biome::Rock) => 0,
                (Biome::Grass, Biome::DarkRock) => 0,
                (Biome::Grass, Biome::Snow) => 0,
                (Biome::DarkGrass, Biome::Desert) => 0,
                (Biome::DarkGrass, Biome::Rock) => 0,
                (Biome::DarkGrass, Biome::DarkRock) => 0,
                (Biome::DarkGrass, Biome::Snow) => 0,
                (Biome::Grass, Biome::DarkGrass) => 0,
                (Biome::Snow, Biome::Rock) => 0,
                (Biome::Water, Biome::DarkRock) => 0,
                (Biome::Desert, Biome::Snow) => 0,
                (Biome::Rock, Biome::Snow) => 0,
                (Biome::DarkRock, Biome::Snow) => 0,
                (_, Biome::Nothing) => 0,
                _ => neighbor.texture_index() * Biome::number_of_combinations() + self.texture_index_for_directions(&directions) + 1
            }
        }        
        0 
    }

    fn texture_index_for_directions(&self, directions: &[Direction]) -> i32 {
        if directions.len() == 1 {
            match directions[0] {
                Direction::Up => return 0,
                Direction::Right => return 1,
                Direction::Down => return 2,
                Direction::Left => return 3,
                _ => {}
            }
        }
        if directions.len() == 2 {
            match (directions[0], directions[1]) {
                (Direction::Up, Direction::Left) => return 4,
                (Direction::Up, Direction::Right) => return 5,
                (Direction::Right, Direction::Down) => return 6,
                (Direction::Down, Direction::Left) => return 7,
                (Direction::Up, Direction::Down) => return 13,
                (Direction::Right, Direction::Left) => return 14,
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
    fn number_of_combinations() -> i32 {
        15
    }

    fn number_of_biomes() -> i32 {
        12
    }

    fn texture_index(&self) -> i32 {
        match self {
            Biome::Water => 0,
            Biome::Desert => 1,
            Biome::Grass => 2,
            Biome::Rock => 3,
            Biome::Snow => 4,
            Biome::LightWood => 5,
            Biome::DarkWood => 6,
            Biome::Nothing => 7,
            Biome::DarkRock => 8,
            Biome::Ice => 9,
            Biome::DarkGrass => 10,
            Biome::RockPlates => 11
        }
    }
}

impl TileSet<BiomeTile> {
    pub fn update_tile(&mut self, row: usize, col: usize, new_biome: Biome) {
        if row >= self.tiles.len() || col >= self.tiles[0].len() {
            return
        }
        self.tiles[row][col].tile_type = new_biome;
        self.tiles[row][col].setup_textures();

        if row > 0 {
            self.tiles[row-1][col].tile_down_type = new_biome;
            self.tiles[row-1][col].setup_textures();
        }
        if row < self.tiles.len() - 1 {
            self.tiles[row+1][col].tile_up_type = new_biome;
            self.tiles[row+1][col].setup_textures();
        }
        if col > 0 {
            self.tiles[row][col-1].tile_right_type = new_biome;
            self.tiles[row][col-1].setup_textures();
        }
        if col < self.tiles[0].len() - 1 {
            self.tiles[row][col+1].tile_left_type = new_biome;
            self.tiles[row][col+1].setup_textures();
        }
    }
}

impl Biome {
    pub const fn from_char(c: char) -> Self {
        match c {
            '0' => Biome::Nothing,
            '1' => Biome::Grass,
            '2' => Biome::Water,
            '3' => Biome::Rock,
            '4' => Biome::Desert,
            '5' => Biome::Snow,
            '6' => Biome::DarkWood,
            '7' => Biome::LightWood,
            '8' => Biome::DarkRock,
            '9' => Biome::Ice,
            'A' => Biome::DarkGrass,
            'B' => Biome::RockPlates,
            _ => Biome::Nothing,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Biome::Nothing => '0',
            Biome::Grass => '1',
            Biome::Water => '2',
            Biome::Rock => '3',
            Biome::Desert => '4',
            Biome::Snow => '5',
            Biome::DarkWood => '6',
            Biome::LightWood => '7',
            Biome::DarkRock => '8',
            Biome::Ice => '9',
            Biome::DarkGrass => 'A',
            Biome::RockPlates => 'B',
        }
    }
}

impl BiomeTile {
    pub fn from_data(row: usize, column: usize, data: char) -> Self {
        let mut tile = Self { 
            tile_type: Biome::from_char(data), 
            column: column as u32, 
            row: row as u32, 
            tile_up_type: Biome::Nothing,
            tile_right_type: Biome::Nothing,
            tile_down_type: Biome::Nothing,
            tile_left_type: Biome::Nothing,
            texture_offset_x: 0, 
            texture_offset_y: 0 
        };
        tile.setup_textures();
        tile
    }
}

impl Serialize for TileSet<BiomeTile> {    
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("TileSet", 2)?;
        let serialized_tiles: Vec<String> = self.tiles.iter().map(|row| {
            row.iter().map(|tile| {
                tile.tile_type.to_char()
            }).collect()
        }).collect();

        state.serialize_field("tiles", &serialized_tiles)?;
        state.serialize_field("sheet_id", &self.sheet_id)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for TileSet<BiomeTile> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        #[derive(Deserialize)]
        struct TileSetData {
            tiles: Vec<String>,
            sheet_id: u32,
        }

        let data = TileSetData::deserialize(deserializer)?;

        let mut tiles: Vec<Vec<BiomeTile>> = data.tiles.into_iter().enumerate().map(|(row, tile_row)| {
            tile_row.chars().enumerate().map(|(column, tile_char)| {
                BiomeTile::from_data(row, column, tile_char)
            }).collect()
        }).collect();

        let rows = tiles.len();
        let columns = if rows > 0 { tiles[0].len() } else { 0 };

        for row in 0..rows {
            for col in 0..columns {
                let up = if row > 0 { tiles[row-1][col].tile_type } else { Biome::Nothing };
                let right = if col < columns - 1 { tiles[row][col+1].tile_type } else { Biome::Nothing };
                let down = if row < rows - 1 { tiles[row+1][col].tile_type } else { Biome::Nothing };
                let left = if col > 0 { tiles[row][col-1].tile_type } else { Biome::Nothing };

                tiles[row][col].setup_neighbors(up, right, down, left)
            }
        }

        Ok(TileSet::with_tiles(data.sheet_id, tiles))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::directions::Direction;

    use super::{Biome, BiomeTile};

    const COLOR_GRASS: u32 = 0x00FF00FF;
    const COLOR_WATER: u32 = 0x0000FFFF;
    const COLOR_ROCK: u32 = 0x7F7F7FFF;
    const COLOR_DESERT: u32 = 0xFFFF00FF;
    const COLOR_SNOW: u32 = 0xFFFFFFFF;
    const COLOR_LIGHT_WOOD: u32 = 0xBF6F4AFF;
    const COLOR_DARK_WOOD: u32 = 0x5D2C28FF;

    impl Biome {
        fn from_color(color: u32) -> Biome {
            match color {
                COLOR_GRASS => Biome::Grass,
                COLOR_WATER => Biome::Water,
                COLOR_ROCK => Biome::Rock,
                COLOR_DESERT => Biome::Desert,
                COLOR_SNOW => Biome::Snow,
                COLOR_LIGHT_WOOD => Biome::LightWood,
                COLOR_DARK_WOOD => Biome::DarkWood,
                _ => Biome::Nothing
            }
        }
    }

    impl BiomeTile {
        fn with_color_indeces(color: u32, column: u32, row: u32) -> Self {
            Self::with_color_indeces_size(color, column, row)
        }
    
        fn with_color_indeces_size(color: u32, column: u32, row: u32) -> Self {
            let tile_type = Biome::from_color(color);
            
            Self {
                tile_type,
                column, 
                row,
                tile_up_type: Biome::Grass,
                tile_right_type: Biome::Grass,
                tile_down_type: Biome::Grass,
                tile_left_type: Biome::Grass,
                texture_offset_x: 0,
                texture_offset_y: 0,
            }
        }
    }
        
    #[test]
    fn can_return_correct_index_from_directions() {
        let tile = BiomeTile::with_color_indeces(COLOR_WATER, 0, 0);
        assert_eq!(tile.texture_index_for_directions(&[Direction::Up]), 0);
        assert_eq!(tile.texture_index_for_directions(&[Direction::Right]), 1);
        assert_eq!(tile.texture_index_for_directions(&[Direction::Down]), 2);
        assert_eq!(tile.texture_index_for_directions(&[Direction::Left]), 3);
        assert_eq!(tile.texture_index_for_directions(&[Direction::Up, Direction::Left]), 4);
        assert_eq!(tile.texture_index_for_directions(&[Direction::Up, Direction::Right]), 5);
        assert_eq!(tile.texture_index_for_directions(&[Direction::Right, Direction::Down]), 6);
        assert_eq!(tile.texture_index_for_directions(&[Direction::Down, Direction::Left]), 7);
        assert_eq!(tile.texture_index_for_directions(&[Direction::Up, Direction::Right, Direction::Down]), 8);
        assert_eq!(tile.texture_index_for_directions(&[Direction::Right, Direction::Down, Direction::Left]), 9);
        assert_eq!(tile.texture_index_for_directions(&[Direction::Up, Direction::Down, Direction::Left]), 10);
        assert_eq!(tile.texture_index_for_directions(&[Direction::Up, Direction::Right, Direction::Left]), 11);
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