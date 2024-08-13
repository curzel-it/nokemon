

/* 
impl World {
    pub fn load_constructions_tiles(&mut self) {
        let mut tiles = parse_constructions_map(WORLD_BIOME_PATH);
        integrate_borders_info(&mut tiles);
        make_obstacles(self, &tiles);
    }
}

fn make_obstacles(world: &mut World, tiles: &Vec<Vec<ConstructionTile>>) {
    for row in tiles {
        let obstacles: Vec<Box<dyn Entity>> = joined_tiles(row)
            .iter()
            .filter(|tile| tile.is_water())
            .map(|tile| tile.into_obstacle_entity(&world.entity_factory))
            .collect();

        for obstacle in obstacles {
            world.add_entity(obstacle);
        };
    }
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
            current.tile_up_type = tile_up_type;
            current.tile_right_type = tile_right_type;
            current.tile_down_type = tile_down_type;
            current.tile_left_type = tile_left_type;
        }
    }
}

fn parse_constructions_map(image_path: &str) -> (u32, u32, Vec<Vec<ConstructionTile>>) {
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

    (width, height, tiles)
}

fn joined_tiles(tiles: &Vec<ConstructionTile>) -> Vec<ConstructionTile> {
    let mut joined: Vec<ConstructionTile> = vec![];    
    let mut previous = tiles[0];
    
    for i in 1..tiles.len() {
        let current = tiles[i];
        
        if current.tile_type == previous.tile_type {
            previous.width += 1;
        } else {
            joined.push(previous);
            previous = current;
        }
    }
    joined.push(previous);

    joined
}*/