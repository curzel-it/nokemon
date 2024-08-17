use image::{GenericImageView, Pixel};

use crate::{constants::{TILE_SIZE, WORLD_MAP_CONSTRUCTIONS}, game_engine::{entity::Entity, obstacles::StaticObstacle, world::World}};

use super::{constructions_tiles::{group_construction_tiles, Construction, ConstructionTile}, tiles::{Tile, TileSet}};
 
impl World {
    pub fn load_constructions_tiles(&mut self) {
        let mut tiles = parse_constructions_map(WORLD_MAP_CONSTRUCTIONS);
        integrate_borders_info(&mut tiles);
        make_obstacles(self, &tiles);
        self.constructions_tiles = TileSet::with_tiles("tiles_constructions".to_owned(), tiles);
    }
}

fn make_obstacles(world: &mut World, tiles: &Vec<Vec<ConstructionTile>>) {
    let flattened_tiles: Vec<ConstructionTile> = tiles.iter().flatten().copied().collect();
    let obstacles: Vec<Box<dyn Entity>> = group_construction_tiles(&flattened_tiles)
        .iter()
        .filter(|t| t.is_something())
        .map(|tile| { tile.into_obstacle_entity() })
        .collect();

    for obstacle in obstacles {
        world.add_entity(obstacle);
    };
}

fn integrate_borders_info(tiles: &mut Vec<Vec<ConstructionTile>>) {
    let rows = tiles.len();
    let columns = tiles[0].len();

    for row in 0..rows {
        for col in 0..columns {
            let mut tile_up_type = Construction::Nothing;
            let mut tile_right_type = Construction::Nothing;
            let mut tile_down_type = Construction::Nothing;
            let mut tile_left_type = Construction::Nothing;

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
            current.setup_neighbors(
                tile_up_type,
                tile_right_type,
                tile_down_type,
                tile_left_type
            );
        }
    }
}

fn parse_constructions_map(image_path: &str) -> Vec<Vec<ConstructionTile>> {
    let img = image::open(image_path).expect("Failed to open image");
    let (width, height) = img.dimensions();

    let mut tiles: Vec<Vec<ConstructionTile>> = Vec::new();

    for y in 0..height {
        let mut row: Vec<ConstructionTile> = Vec::new();

        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            let color_int = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);            
            let tile = ConstructionTile::with_color_indeces(color_int, x, y);
            row.push(tile);
        }
        tiles.push(row);
    }

    tiles
}

impl ConstructionTile {
    fn into_obstacle_entity(&self) -> Box<dyn Entity> {
        println!("type {:#?}, col {}, row {}, w {}, h {}", self.tile_type, self.column, self.row, self.width, self.height);
        let entity = StaticObstacle::new(
            self.tile_type.sprite(),
            raylib::math::Rectangle::new(
                self.column as f32 * TILE_SIZE, 
                self.row as f32 * TILE_SIZE, 
                self.width as f32 * TILE_SIZE, 
                self.height as f32 * TILE_SIZE
            )
        );
        Box::new(entity)
    }
}