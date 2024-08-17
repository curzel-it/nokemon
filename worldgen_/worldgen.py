import pdb
import sys
import json
from PIL import Image
from collections import defaultdict
import unittest

# Constants for the biome colors
COLOR_GRASS = 0x00FF00
COLOR_WATER = 0x0000FF
COLOR_ROCK = 0x7F7F7F
COLOR_DESERT = 0xFFFF00
COLOR_SNOW = 0xFFFFFF

class Direction:
    Up = "Up"
    Right = "Right"
    Down = "Down"
    Left = "Left"

class Biome:
    GRASS = "Grass"
    WATER = "Water"
    ROCK = "Rock"
    DESERT = "Desert"
    SNOW = "Snow"

    @staticmethod
    def from_color(color):
        if color == COLOR_GRASS:
            return Biome.GRASS
        elif color == COLOR_WATER:
            return Biome.WATER
        elif color == COLOR_ROCK:
            return Biome.ROCK
        elif color == COLOR_DESERT:
            return Biome.DESERT
        elif color == COLOR_SNOW:
            return Biome.SNOW
        return Biome.DESERT  # Default to Desert if no match

class BiomeTile:
    def __init__(self, tile_type, column, row, width=1, height=1, 
                 tile_up_type=None, tile_right_type=None, tile_down_type=None, tile_left_type=None):
        self.tile_type = tile_type
        self.column = column
        self.row = row
        self.width = width
        self.height = height
        self.tile_up_type = tile_up_type or tile_type
        self.tile_right_type = tile_right_type or tile_type
        self.tile_down_type = tile_down_type or tile_type
        self.tile_left_type = tile_left_type or tile_type

    def is_water(self):
        return self.tile_type == Biome.WATER

    def to_dict(self):
        return {
            "tile_type": self.tile_type,
            "column": self.column,
            "row": self.row,
            "width": self.width,
            "height": self.height,
            "tile_up_type": self.tile_up_type,
            "tile_right_type": self.tile_right_type,
            "tile_down_type": self.tile_down_type,
            "tile_left_type": self.tile_left_type
        }

def parse_biome_map(image_path):
    img = Image.open(image_path)
    width, height = img.size
    pixels = img.load()

    tiles = []
    for y in range(height):
        row = []
        for x in range(width):
            color = (pixels[x, y][0] << 16) | (pixels[x, y][1] << 8) | pixels[x, y][2]
            tile = BiomeTile(Biome.from_color(color), x, y)
            row.append(tile)
        tiles.append(row)
    
    return width, height, tiles

def integrate_borders_info(tiles):
    rows = len(tiles)
    columns = len(tiles[0])

    for row in range(rows):
        for col in range(columns):
            current = tiles[row][col]
            tile_up_type = tiles[row-1][col].tile_type if row > 0 else current.tile_type
            tile_right_type = tiles[row][col+1].tile_type if col < columns - 1 else current.tile_type
            tile_down_type = tiles[row+1][col].tile_type if row < rows - 1 else current.tile_type
            tile_left_type = tiles[row][col-1].tile_type if col > 0 else current.tile_type

            current.tile_up_type = tile_up_type
            current.tile_right_type = tile_right_type
            current.tile_down_type = tile_down_type
            current.tile_left_type = tile_left_type

def group_biome_tiles(tiles):
    result = []
    visited = set()

    rows = max(tile.row for tile in tiles) + 1
    cols = max(tile.column for tile in tiles) + 1

    for tile in tiles:
        if (tile.row, tile.column) in visited:
            continue

        max_width = 1
        max_height = 1

        # Calculate max width
        while tile.column + max_width < cols and any(
            t.row == tile.row and t.column == tile.column + max_width and t.tile_type == tile.tile_type for t in tiles
        ):
            max_width += 1

        # Calculate max height
        valid_height = True
        while valid_height and tile.row + max_height < rows:
            for col_offset in range(max_width):
                if not any(
                    t.row == tile.row + max_height and t.column == tile.column + col_offset and t.tile_type == tile.tile_type for t in tiles
                ):
                    valid_height = False
                    break
            if valid_height:
                max_height += 1

        # Mark tiles as visited
        for row_offset in range(max_height):
            for col_offset in range(max_width):
                visited.add((tile.row + row_offset, tile.column + col_offset))

        # Create the grouped tile
        grouped_tile = BiomeTile(
            tile_type=tile.tile_type,
            column=tile.column,
            row=tile.row,
            width=max_width,
            height=max_height,
            tile_up_type=tile.tile_type,
            tile_right_type=tile.tile_type,
            tile_down_type=tile.tile_type,
            tile_left_type=tile.tile_type
        )
        result.append(grouped_tile)

    return result

def save_to_json(tiles, output_path):
    tiles_dict = [tile.to_dict() for tile in tiles]
    with open(output_path, 'w') as f:
        json.dump(tiles_dict, f, indent=4)

def main():
    image_path = "../levels/world_biome.png"
    output_path = "../levels/world_biome.json"

    width, height, tiles = parse_biome_map(image_path)
    integrate_borders_info(tiles)
    grouped_tiles = group_biome_tiles([tile for row in tiles for tile in row])

    save_to_json(grouped_tiles, output_path)
    print(f"Biome tiles saved to {output_path}")

# Unit tests
class TestBiomeTileFunctions(unittest.TestCase):
    def test_can_group_l_and_square(self):
        tiles_info = [
            [0, 1, 1],
            [0, 1, 1],
            [0, 0, 0],
        ]
        tiles = tiles_from_indeces(tiles_info)
        grouped_tiles = group_biome_tiles(tiles)

        self.assertEqual(grouped_tiles[0].tile_type, Biome.GRASS)
        self.assertEqual(grouped_tiles[0].width, 1)
        self.assertEqual(grouped_tiles[0].height, 3)

        self.assertEqual(grouped_tiles[1].tile_type, Biome.WATER)
        self.assertEqual(grouped_tiles[1].width, 2)
        self.assertEqual(grouped_tiles[1].height, 2)

        self.assertEqual(grouped_tiles[2].tile_type, Biome.GRASS)
        self.assertEqual(grouped_tiles[2].width, 2)
        self.assertEqual(grouped_tiles[2].height, 1)

    def test_can_group_vertical_strips(self):
        tiles_info = [
            [0, 1, 2],
            [0, 1, 2],
            [0, 1, 2],
        ]
        tiles = tiles_from_indeces(tiles_info)
        grouped_tiles = group_biome_tiles(tiles)

        self.assertEqual(grouped_tiles[0].tile_type, Biome.GRASS)
        self.assertEqual(grouped_tiles[0].width, 1)
        self.assertEqual(grouped_tiles[0].height, 3)

        self.assertEqual(grouped_tiles[1].tile_type, Biome.WATER)
        self.assertEqual(grouped_tiles[1].width, 1)
        self.assertEqual(grouped_tiles[1].height, 3)

        self.assertEqual(grouped_tiles[2].tile_type, Biome.DESERT)
        self.assertEqual(grouped_tiles[2].width, 1)
        self.assertEqual(grouped_tiles[2].height, 3)

    def test_can_group_and_leave_islands(self):
        tiles_info = [
            [0, 1, 1],
            [2, 1, 1],
            [0, 2, 0],
        ]
        tiles = tiles_from_indeces(tiles_info)
        grouped_tiles = group_biome_tiles(tiles)

        self.assertEqual(grouped_tiles[0].tile_type, Biome.GRASS)
        self.assertEqual(grouped_tiles[0].width, 1)
        self.assertEqual(grouped_tiles[0].height, 1)

        self.assertEqual(grouped_tiles[1].tile_type, Biome.WATER)
        self.assertEqual(grouped_tiles[1].width, 2)
        self.assertEqual(grouped_tiles[1].height, 2)

        self.assertEqual(grouped_tiles[2].tile_type, Biome.DESERT)
        self.assertEqual(grouped_tiles[2].width, 1)
        self.assertEqual(grouped_tiles[2].height, 1)

        self.assertEqual(grouped_tiles[3].tile_type, Biome.GRASS)
        self.assertEqual(grouped_tiles[3].width, 1)
        self.assertEqual(grouped_tiles[3].height, 1)

        self.assertEqual(grouped_tiles[4].tile_type, Biome.DESERT)
        self.assertEqual(grouped_tiles[4].width, 1)
        self.assertEqual(grouped_tiles[4].height, 1)

        self.assertEqual(grouped_tiles[5].tile_type, Biome.GRASS)
        self.assertEqual(grouped_tiles[5].width, 1)
        self.assertEqual(grouped_tiles[5].height, 1)

def tiles_from_indeces(items):
    tiles = []
    for row_index, row in enumerate(items):
        for col_index, item in enumerate(row):
            tile_type = Biome.GRASS
            if item == 1:
                tile_type = Biome.WATER
            elif item == 2:
                tile_type = Biome.DESERT
            tiles.append(BiomeTile(tile_type, col_index, row_index))
    return tiles

if __name__ == "__main__":
    if "test" in sys.argv:
        sys.argv = [a for a in sys.argv if a != "test"]
        unittest.main()
    else:
        main()
