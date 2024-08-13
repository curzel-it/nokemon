import sys
import json
from PIL import Image

COLOR_TILES = {
    (0, 255, 0): "Grass",
    (0, 0, 255): "Water",
    (127, 127, 127): "Rock",
    (255, 255, 0): "Desert",
    (255, 255, 255): "Snow",
}
TILE_COLORS = res = dict((v,k) for k,v in COLOR_TILES.items())

def load_map(json_path):
    with open(json_path, 'r') as file:
        tiles = json.load(file)
    
    columns = max(tile['column'] for tile in tiles) + 1
    rows = max(tile['row'] for tile in tiles) + 1
    
    tile_matrix = [[None for _ in range(columns)] for _ in range(rows)]
    
    for tile in tiles:
        tile_matrix[tile['row']][tile['column']] = tile['tile_type']
    
    return tile_matrix, rows, columns

def generate_bitmap(tile_matrix, rows, columns, tile_size=1):
    # Create a new image with the correct size
    img = Image.new('RGB', (columns * tile_size, rows * tile_size))
    
    for row in range(rows):
        for col in range(columns):
            tile_type = tile_matrix[row][col]
            color = TILE_COLORS.get(tile_type, (0, 0, 0))  # Default to black if unknown
            
            # Draw the tile as a square of tile_size
            for i in range(tile_size):
                for j in range(tile_size):
                    img.putpixel((col * tile_size + i, row * tile_size + j), color)
    
    return img

def load_image(image_path):
    return Image.open(image_path)

def generate_json(img, tile_size=1):
    rows = img.height // tile_size
    columns = img.width // tile_size
    
    tiles = []

    for row in range(rows):
        for col in range(columns):
            # Extract the color of the top-left pixel of the tile (or any pixel inside it)
            pixel_color = img.getpixel((col * tile_size, row * tile_size))
            
            # Get the tile type corresponding to the color
            if len(pixel_color) == 4:
                pixel_color = pixel_color[:3]

            tile_type = COLOR_TILES.get(pixel_color, "Unknown")
            
            # Create a tile info dictionary
            tile_info = {
                "tile_type": tile_type,
                "column": col,
                "row": row,
            }
            
            tiles.append(tile_info)
    
    return tiles

def save_json(tiles, json_path):
    with open(json_path, 'w') as json_file:
        json.dump(tiles, json_file, indent=4)

def json_to_png():
    # Load the map
    json_path = '../world.json'
    tile_matrix, rows, columns = load_map(json_path)
    
    # Generate the bitmap image
    img = generate_bitmap(tile_matrix, rows, columns)
    
    # Save or show the image
    img.save('../world.png')
    img.show()

def png_to_json():
    # Load the image
    image_path = '../world.png'
    img = load_image(image_path)
    
    # Generate the JSON data
    tiles = generate_json(img)
    
    # Save the JSON data to a file
    json_path = '../world.json'
    save_json(tiles, json_path)
    print(f"Map saved to {json_path}")

if __name__ == "__main__":
    if "image" in sys.argv:
        json_to_png()
    else:
        png_to_json()
