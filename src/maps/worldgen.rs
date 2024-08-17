use bincode;
use image::{GenericImageView, Pixel};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashSet;

pub const WORLD_MAP_BIOME: &str = "/Users/curzel/dev/tower-defense/levels/world_biome.png";
pub const WORLD_MAP_BIOME_BIN: &str = "/Users/curzel/dev/tower-defense/levels/world_biome.bin";
pub const WORLD_MAP_CONSTRUCTIONS: &str = "/Users/curzel/dev/tower-defense/levels/world_constructions.png";
pub const WORLD_MAP_CONSTRUCTIONS_BIN: &str = "/Users/curzel/dev/tower-defense/levels/world_constructions.bin";

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct TileItem {
    pub tile_type: u32,
    pub column: u32,
    pub row: u32,
    pub width: u32,
    pub height: u32,
    pub tile_up_type: u32,
    pub tile_right_type: u32,
    pub tile_down_type: u32,
    pub tile_left_type: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TiledMap {
    pub tiles_matrix: Vec<Vec<TileItem>>,
    pub grouped_tiles: Vec<TileItem>
}

pub fn create_map_binaries() {
    let layers = vec![
        (WORLD_MAP_BIOME, WORLD_MAP_BIOME_BIN),
        (WORLD_MAP_CONSTRUCTIONS, WORLD_MAP_CONSTRUCTIONS_BIN),
    ];
    for (image_path, output_path) in layers {
        create_map_binary(image_path, output_path)
    }
}

fn create_map_binary(image_path: &str, output_path: &str) {
    let mut tiles_matrix = parse_map_image(image_path);
    
    integrate_borders_info(&mut tiles_matrix);

    let flattened_tiles: Vec<TileItem> = tiles_matrix
        .iter()
        .flatten()
        .copied()
        .collect();
    
    let grouped_tiles = group_tiles(&flattened_tiles);

    let map = TiledMap { tiles_matrix, grouped_tiles };
    create_map_binary_for_map(&map, output_path);
}

pub fn create_map_binary_for_map(map: &TiledMap, output_path: &str) {
    let mut file = File::create(output_path).expect("Failed to create output file");
    let binary_data = bincode::serialize(&map).expect("Failed to serialize tiles to binary");
    file.write_all(&binary_data).expect("Failed to write binary data to file");
    println!("Data successfully written to {}", output_path);
}

pub fn deserialize_tiled_map(file_path: &str) -> TiledMap {
    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    bincode::deserialize(&buffer).unwrap()
}

fn parse_map_image(image_path: &str) -> Vec<Vec<TileItem>> {
    let img = image::open(image_path).expect("Failed to open image");
    let (width, height) = img.dimensions();

    let mut tiles: Vec<Vec<TileItem>> = Vec::new();

    for y in 0..height {
        let mut row: Vec<TileItem> = Vec::new();

        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgba();
            let tile_type =  (pixel[0] as u32) << 24 | (pixel[1] as u32) << 16 | (pixel[2] as u32) << 8 | (pixel[3] as u32);

            let tile = TileItem {
                tile_type,
                column: x,
                row: y,
                width: 1,
                height: 1,
                tile_up_type: 0,
                tile_right_type: 0,
                tile_down_type: 0,
                tile_left_type: 0
            };
            row.push(tile);
        }
        tiles.push(row);
    }

    tiles
}

pub fn group_tiles(tiles: &Vec<TileItem>) -> Vec<TileItem> {
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

        let group = TileItem {
            tile_type: tile.tile_type,
            column: tile.column,
            row: tile.row,
            width: max_width,
            height: max_height,
            tile_up_type: tile.tile_type,
            tile_right_type: tile.tile_type,
            tile_down_type: tile.tile_type,
            tile_left_type: tile.tile_type
        };
        result.push(group);
    }

    result
}

fn integrate_borders_info(tiles: &mut Vec<Vec<TileItem>>) {
    let rows = tiles.len();
    let columns = tiles[0].len();

    for row in 0..rows {
        for col in 0..columns {
            let current_biome = tiles[row][col].tile_type;
            let mut tile_up_type = current_biome;
            let mut tile_right_type = current_biome;
            let mut tile_down_type = current_biome;
            let mut tile_left_type = current_biome;

            if row > 0 {
                tile_up_type = tiles[row-1][col].tile_type;
            }
            if col < columns - 1 {
                tile_right_type = tiles[row][col+1].tile_type;
            }
            if row < rows - 1 {
                tile_down_type = tiles[row+1][col].tile_type;
            }
            if col > 0 {
                tile_left_type = tiles[row][col-1].tile_type;
            }

            let current = &mut tiles[row][col];
            current.tile_up_type = tile_up_type;
            current.tile_right_type = tile_right_type;
            current.tile_down_type = tile_down_type;
            current.tile_left_type = tile_left_type;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::maps::constructions_tiles::COLOR_WOODEN_FENCE;

    use super::{create_map_binary_for_map, deserialize_tiled_map, group_tiles, TileItem, TiledMap};

    fn tiles_from_indeces(items: Vec<Vec<i32>>) -> Vec<TileItem> {
        items
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row
                    .iter()
                    .enumerate()
                    .map(move |(col_index, item)| {
                        TileItem {
                            tile_type: *item as u32,
                            column: col_index as u32,
                            row: row_index as u32,
                            width: 1,
                            height: 1,
                            tile_up_type: 0,
                            tile_right_type: 0,
                            tile_down_type: 0,
                            tile_left_type: 0
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
        let grouped_tiles = group_tiles(&tiles);
        
        assert_eq!(grouped_tiles[0].tile_type, 0);
        assert_eq!(grouped_tiles[0].width, 1);
        assert_eq!(grouped_tiles[0].height, 3);
        
        assert_eq!(grouped_tiles[1].tile_type, 1);
        assert_eq!(grouped_tiles[1].width, 2);
        assert_eq!(grouped_tiles[1].height, 2);
        
        assert_eq!(grouped_tiles[2].tile_type, 0);
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
        let grouped_tiles = group_tiles(&tiles);
        
        assert_eq!(grouped_tiles[0].tile_type, 0);
        assert_eq!(grouped_tiles[0].width, 1);
        assert_eq!(grouped_tiles[0].height, 3);
        
        assert_eq!(grouped_tiles[1].tile_type, 1);
        assert_eq!(grouped_tiles[1].width, 1);
        assert_eq!(grouped_tiles[1].height, 3);
        
        assert_eq!(grouped_tiles[2].tile_type, 2);
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
        let grouped_tiles = group_tiles(&tiles);
        
        assert_eq!(grouped_tiles[0].tile_type, 0);
        assert_eq!(grouped_tiles[0].width, 1);
        assert_eq!(grouped_tiles[0].height, 1);
        
        assert_eq!(grouped_tiles[1].tile_type, 1);
        assert_eq!(grouped_tiles[1].width, 2);
        assert_eq!(grouped_tiles[1].height, 2);
        
        assert_eq!(grouped_tiles[2].tile_type, 2);
        assert_eq!(grouped_tiles[2].width, 1);
        assert_eq!(grouped_tiles[2].height, 1);

        assert_eq!(grouped_tiles[3].tile_type, 0);
        assert_eq!(grouped_tiles[3].width, 1);
        assert_eq!(grouped_tiles[3].height, 1);
        
        assert_eq!(grouped_tiles[4].tile_type, 2);
        assert_eq!(grouped_tiles[4].width, 1);
        assert_eq!(grouped_tiles[4].height, 1);

        assert_eq!(grouped_tiles[5].tile_type, 0);
        assert_eq!(grouped_tiles[5].width, 1);
        assert_eq!(grouped_tiles[5].height, 1);
    }

    #[test]
    fn can_encode_and_decode() {
        let tiles_matrix = vec![
            (40..44).map(|column| {
                TileItem {
                    tile_type: COLOR_WOODEN_FENCE,
                    column: column,
                    row: 26,
                    width: 1,
                    height: 1,
                    tile_up_type: 0,
                    tile_right_type: COLOR_WOODEN_FENCE,
                    tile_down_type: 0,
                    tile_left_type: 0
                }
            })
            .collect()
        ];

        let grouped_tiles = vec![
            TileItem {
                tile_type: COLOR_WOODEN_FENCE,
                column: 40,
                row: 26,
                width: 4,
                height: 1,
                tile_up_type: 0,
                tile_right_type: 0,
                tile_down_type: 0,
                tile_left_type: 0
            }
        ];

        let path = "test_can_encode_and_decode.bin";
        let original_map = TiledMap { tiles_matrix, grouped_tiles };
        create_map_binary_for_map(&original_map, path);
        
        let decoded_map = deserialize_tiled_map(path);

        assert_eq!(decoded_map.tiles_matrix.len(), original_map.tiles_matrix.len());
        assert_eq!(decoded_map.tiles_matrix[0].len(), original_map.tiles_matrix[0].len());
        assert_eq!(decoded_map.grouped_tiles.len(), original_map.grouped_tiles.len());

        println!("Colors check {:08X} vs {:08X}", decoded_map.tiles_matrix[0][0].tile_type, original_map.tiles_matrix[0][0].tile_type);
        println!("Colors check {:08X} vs {:08X}", decoded_map.grouped_tiles[0].tile_type, original_map.grouped_tiles[0].tile_type);

        assert_eq!(decoded_map.tiles_matrix[0][0].tile_type, original_map.tiles_matrix[0][0].tile_type);
        assert_eq!(decoded_map.grouped_tiles[0].tile_type, original_map.grouped_tiles[0].tile_type);
    }
}

