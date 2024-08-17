import sys
import unittest
import json
from PIL import Image

# Constants for biome colors
COLOR_GRASS = 0x00FF00
COLOR_WATER = 0x0000FF
COLOR_ROCK = 0x7F7F7F
COLOR_DESERT = 0xFFFF00
COLOR_SNOW = 0xFFFFFF

# Biome types
class Biome:
    Grass = "grass"
    Water = "water"
    Rock = "rock"
    Desert = "desert"
    Snow = "snow"

    @staticmethod
    def from_color(color):
        if color == COLOR_GRASS:
            return Biome.Grass
        elif color == COLOR_WATER:
            return Biome.Water
        elif color == COLOR_ROCK:
            return Biome.Rock
        elif color == COLOR_DESERT:
            return Biome.Desert
        elif color == COLOR_SNOW:
            return Biome.Snow
        else:
            return Biome.Desert

# Biome Tile class
class BiomeTile:
    def __init__(self, tile_type, column, row, width=1, height=1):
        self.tile_type = tile_type
        self.column = column
        self.row = row
        self.width = width
        self.height = height
        self.tile_up_type = tile_type
        self.tile_right_type = tile_type
        self.tile_down_type = tile_type
        self.tile_left_type = tile_type

    def setup_neighbors(self, up, right, bottom, left):
        self.tile_up_type = up
        self.tile_right_type = right
        self.tile_down_type = bottom
        self.tile_left_type = left

    def is_water(self):
        return self.tile_type == Biome.Water

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

    tiles = []
    for y in range(height):
        row = []
        for x in range(width):
            pixel = img.getpixel((x, y))
            color_int = (pixel[0] << 16) | (pixel[1] << 8) | pixel[2]
            tile_type = Biome.from_color(color_int)
            tile = BiomeTile(tile_type, x, y)
            row.append(tile)
        tiles.append(row)

    return width, height, tiles

def integrate_borders_info(tiles):
    rows = len(tiles)
    columns = len(tiles[0])

    for row in range(rows):
        for col in range(columns):
            current_biome = tiles[row][col].tile_type
            tile_up_type = current_biome
            tile_right_type = current_biome
            tile_down_type = current_biome
            tile_left_type = current_biome

            if row > 0:
                tile_up_type = tiles[row-1][col].tile_type
            if col < columns - 1:
                tile_right_type = tiles[row][col+1].tile_type
            if row < rows - 1:
                tile_down_type = tiles[row+1][col].tile_type
            if col > 0:
                tile_left_type = tiles[row][col-1].tile_type

            tiles[row][col].setup_neighbors(
                tile_up_type,
                tile_right_type,
                tile_down_type,
                tile_left_type
            )

def group_biome_tiles(tiles):
    result = []
    visited = set()

    rows = len(tiles)
    cols = len(tiles[0])

    for row in tiles:
        for tile in row:
            if (tile.row, tile.column) in visited:
                continue

            max_width = 1
            max_height = 1

            while tile.column + max_width < cols and tiles[tile.row][tile.column + max_width].tile_type == tile.tile_type:
                max_width += 1

            valid_height = True
            while valid_height and tile.row + max_height < rows:
                for col_offset in range(max_width):
                    if tiles[tile.row + max_height][tile.column + col_offset].tile_type != tile.tile_type:
                        valid_height = False
                        break
                if valid_height:
                    max_height += 1

            for row_offset in range(max_height):
                for col_offset in range(max_width):
                    visited.add((tile.row + row_offset, tile.column + col_offset))

            group = BiomeTile(tile.tile_type, tile.column, tile.row, max_width, max_height)
            result.append(group)

    return result

def make_water_obstacles(tiles):
    water_tiles = [tile for row in tiles for tile in row if tile.is_water()]
    return group_biome_tiles(water_tiles)

def save_to_json(tiles, file_name="biome_map.json"):
    data = {
        "tiles": [tile.to_dict() for row in tiles for tile in row]
    }
    with open(file_name, "w") as f:
        json.dump(data, f, indent=4)

def main():
    image_path = "path/to/your/biome_map_image.png"
    _, _, tiles = parse_biome_map(image_path)
    integrate_borders_info(tiles)
    save_to_json(tiles)

class TestBiomeProcessor(unittest.TestCase):
    def test_parse_biome_map(self):
        width, height, tiles = parse_biome_map("test_image.png")
        self.assertEqual(width, 3)
        self.assertEqual(height, 3)
        self.assertEqual(tiles[0][0].tile_type, Biome.Grass)

    def test_integrate_borders_info(self):
        _, _, tiles = parse_biome_map("test_image.png")
        integrate_borders_info(tiles)
        self.assertEqual(tiles[0][0].tile_up_type, Biome.Grass)

    def test_group_biome_tiles(self):
        _, _, tiles = parse_biome_map("test_image.png")
        grouped_tiles = group_biome_tiles(tiles)
        self.assertGreater(len(grouped_tiles), 0)


if __name__ == "__main__":
    if "test" in sys.argv:
        sys.argv = [a for a in sys.argv if a != "test"]
        unittest.main()
    else:
        main()