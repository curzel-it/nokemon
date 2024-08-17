use std::collections::HashSet;

use raylib::math::Rectangle;

use crate::{
    constants::TILE_TEXTURE_SIZE,
    impl_tile,
};

use super::tiles::SpriteTile;

pub const COLOR_WOODEN_FENCE: u32 = 0x391f21;
pub const COLOR_HOUSE: u32 = 0xff00ff;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Construction {
    WoodenFence,
    House,
    Nothing,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstructionTile {
    pub tile_type: Construction,
    pub column: u32,
    pub row: u32,
    pub width: u32,
    pub height: u32,
    pub tile_up_type: Construction,
    pub tile_right_type: Construction,
    pub tile_down_type: Construction,
    pub tile_left_type: Construction,
    pub texture_offset_x: f32,
    pub texture_offset_y: f32,
}

impl Default for ConstructionTile {
    fn default() -> Self {
        ConstructionTile {
            tile_type: Construction::Nothing,
            column: 0,
            row: 0,
            width: 1,
            height: 1,
            tile_up_type: Construction::Nothing,
            tile_right_type: Construction::Nothing,
            tile_down_type: Construction::Nothing,
            tile_left_type: Construction::Nothing,
            texture_offset_x: 0.0,
            texture_offset_y: 0.0,
        }
    }
}

impl ConstructionTile {
    pub fn with_color_indeces(color: u32, column: u32, row: u32) -> Self {
        Self::with_color_indeces_size(color, column, row, 1, 1)
    }

    pub fn with_color_indeces_size(
        color: u32,
        column: u32,
        row: u32,
        width: u32,
        height: u32,
    ) -> Self {
        let tile_type = Construction::from_color(color).unwrap_or(Construction::Nothing);

        Self {
            tile_type,
            column,
            row,
            width,
            height,
            tile_up_type: Construction::Nothing,
            tile_right_type: Construction::Nothing,
            tile_down_type: Construction::Nothing,
            tile_left_type: Construction::Nothing,
            texture_offset_x: tile_type.texture_offset_x() as f32,
            texture_offset_y: 0.0,
        }
    }

    pub fn is_something(&self) -> bool {
        self.tile_type != Construction::Nothing
    }
}

impl_tile!(ConstructionTile);

impl SpriteTile for ConstructionTile {
    fn texture_source_rect(&self, _: u32) -> Rectangle {
        Rectangle::new(
            self.texture_offset_x,
            self.texture_offset_y,
            TILE_TEXTURE_SIZE,
            TILE_TEXTURE_SIZE,
        )
    }
}

impl ConstructionTile {
    pub fn setup_neighbors(
        &mut self,
        up: Construction,
        right: Construction,
        bottom: Construction,
        left: Construction,
    ) {
        self.tile_up_type = up;
        self.tile_right_type = right;
        self.tile_down_type = bottom;
        self.tile_left_type = left;

        if self.tile_type == Construction::WoodenFence { self.setup_tile() }
    }

    fn setup_tile(&mut self) {
        let same_up = self.tile_up_type == self.tile_type;
        let same_right = self.tile_right_type == self.tile_type;
        let same_down = self.tile_down_type == self.tile_type;
        let same_left = self.tile_left_type == self.tile_type;

        let x = self.tile_type.texture_offset_x();
        let y = match (same_up, same_right, same_down, same_left) {
            (false, true, false, true) => 0,
            (false, false, false, false) => 1,
            (false, false, false, true) => 2,
            (false, true, false, false) => 3,
            (true, false, true, false) => 4,
            (true, false, false, false) => 5,
            (false, false, true, false) => 6,
            (true, true, false, false) => 7,
            (true, false, false, true) => 8,
            (false, true, true, false) => 9,
            (false, false, true, true) => 10,
            _ => 0,
        };
        self.texture_offset_x = TILE_TEXTURE_SIZE * x as f32;
        self.texture_offset_y = TILE_TEXTURE_SIZE * y as f32;
    }
}

impl Construction {
    pub fn sprite(&self) -> &str {
        match self {
            Construction::Nothing => "invisible",
            Construction::WoodenFence => "invisible",
            Construction::House => "house",
        }
    }

    fn texture_offset_x(&self) -> u32 {
        match self {
            Construction::Nothing => 0,
            Construction::WoodenFence => 1,
            Construction::House => 0,
        }
    }

    fn from_color(color: u32) -> Option<Construction> {
        match color {
            COLOR_WOODEN_FENCE => Some(Construction::WoodenFence),
            COLOR_HOUSE => Some(Construction::House),
            _ => None,
        }
    }
}

pub fn group_construction_tiles(tiles: &Vec<ConstructionTile>) -> Vec<ConstructionTile> {
    let mut result = Vec::new();
    let mut visited = HashSet::new(); 

    let rows = tiles.iter().map(|t| t.row).max().unwrap_or(0) + 1;
    let cols = tiles.iter().map(|t| t.column).max().unwrap_or(0) + 1;

    for tile in tiles {
        if visited.contains(&(tile.row, tile.column)) {
            continue; 
        }

        let mut max_width = 1;
        let mut max_height = 1;

        while tile.column + max_width < cols
            && tiles.iter().any(|t| {
                t.row == tile.row
                    && t.column == tile.column + max_width
                    && t.tile_type == tile.tile_type
            })
        {
            max_width += 1;
        }

        let mut valid_height = true;
        while valid_height && tile.row + max_height < rows {
            for col_offset in 0..max_width {
                if !tiles.iter().any(|t| {
                    t.row == tile.row + max_height
                        && t.column == tile.column + col_offset
                        && t.tile_type == tile.tile_type
                }) {
                    valid_height = false;
                    break;
                }
            }
            if valid_height {
                max_height += 1;
            }
        }

        for row_offset in 0..max_height {
            for col_offset in 0..max_width {
                visited.insert((tile.row + row_offset, tile.column + col_offset));
            }
        }

        let group = ConstructionTile {
            tile_type: tile.tile_type,
            column: tile.column,
            row: tile.row,
            width: max_width,
            height: max_height,
            tile_up_type: tile.tile_type,
            tile_right_type: tile.tile_type,
            tile_down_type: tile.tile_type,
            tile_left_type: tile.tile_type,
            texture_offset_x: 0.0,
            texture_offset_y: 0.0,
        };
        result.push(group);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{group_construction_tiles, Construction, ConstructionTile};

    impl Construction {
        fn from_index(index: u32) -> Self {
            match index {
                1 => Construction::WoodenFence,
                2 => Construction::House,
                _ => Construction::Nothing
            }
        }
    }

    fn tiles_from_indeces(items: Vec<Vec<i32>>) -> Vec<ConstructionTile> {
        items
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row
                    .iter()
                    .enumerate()
                    .map(move |(col_index, item)| {
                        ConstructionTile {
                            tile_type: Construction::from_index(*item as u32),
                            column: col_index as u32,
                            row: row_index as u32,
                            width: 1,
                            height: 1,
                            tile_up_type: Construction::Nothing,
                            tile_right_type: Construction::Nothing,
                            tile_down_type: Construction::Nothing,
                            tile_left_type: Construction::Nothing,
                            texture_offset_x: 0.0,
                            texture_offset_y: 0.0,
                        }
                    })
            })
            .collect()
    }

    #[test]
    fn can_group_l_and_square() {
        let tiles_info = vec![
            vec![0, 1, 1],
            vec![0, 1, 1],
            vec![0, 0, 0],
        ];
        let tiles = tiles_from_indeces(tiles_info);
        let grouped_tiles = group_construction_tiles(&tiles);
        
        assert_eq!(grouped_tiles[0].tile_type, Construction::Nothing);
        assert_eq!(grouped_tiles[0].width, 1);
        assert_eq!(grouped_tiles[0].height, 3);
        
        assert_eq!(grouped_tiles[1].tile_type, Construction::WoodenFence);
        assert_eq!(grouped_tiles[1].width, 2);
        assert_eq!(grouped_tiles[1].height, 2);
        
        assert_eq!(grouped_tiles[2].tile_type, Construction::Nothing);
        assert_eq!(grouped_tiles[2].width, 2);
        assert_eq!(grouped_tiles[2].height, 1);
    }

    #[test]
    fn can_group_vertical_strips() {
        let tiles_info = vec![
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![0, 1, 2],
        ];
        let tiles = tiles_from_indeces(tiles_info);
        let grouped_tiles = group_construction_tiles(&tiles);
        
        assert_eq!(grouped_tiles[0].tile_type, Construction::Nothing);
        assert_eq!(grouped_tiles[0].width, 1);
        assert_eq!(grouped_tiles[0].height, 3);
        
        assert_eq!(grouped_tiles[1].tile_type, Construction::WoodenFence);
        assert_eq!(grouped_tiles[1].width, 1);
        assert_eq!(grouped_tiles[1].height, 3);
        
        assert_eq!(grouped_tiles[2].tile_type, Construction::House);
        assert_eq!(grouped_tiles[2].width, 1);
        assert_eq!(grouped_tiles[2].height, 3);
    }

    #[test]
    fn can_group_and_leave_islands() {
        let tiles_info = vec![
            vec![0, 1, 1],
            vec![2, 1, 1],
            vec![0, 2, 0],
        ];
        let tiles = tiles_from_indeces(tiles_info);
        let grouped_tiles = group_construction_tiles(&tiles);
        
        assert_eq!(grouped_tiles[0].tile_type, Construction::Nothing);
        assert_eq!(grouped_tiles[0].width, 1);
        assert_eq!(grouped_tiles[0].height, 1);
        
        assert_eq!(grouped_tiles[1].tile_type, Construction::WoodenFence);
        assert_eq!(grouped_tiles[1].width, 2);
        assert_eq!(grouped_tiles[1].height, 2);
        
        assert_eq!(grouped_tiles[2].tile_type, Construction::House);
        assert_eq!(grouped_tiles[2].width, 1);
        assert_eq!(grouped_tiles[2].height, 1);

        assert_eq!(grouped_tiles[3].tile_type, Construction::Nothing);
        assert_eq!(grouped_tiles[3].width, 1);
        assert_eq!(grouped_tiles[3].height, 1);
        
        assert_eq!(grouped_tiles[4].tile_type, Construction::House);
        assert_eq!(grouped_tiles[4].width, 1);
        assert_eq!(grouped_tiles[4].height, 1);

        assert_eq!(grouped_tiles[5].tile_type, Construction::Nothing);
        assert_eq!(grouped_tiles[5].width, 1);
        assert_eq!(grouped_tiles[5].height, 1);
    }
}
