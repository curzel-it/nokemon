import pdb
from PIL import Image
import sys
import os
import subprocess

tile_size = 16
aseprite_path = "/Applications/Aseprite.app/Contents/MacOS/aseprite"
biomes = "grass desert rock snow water".split(" ")
aseprite_assets = "../aseprite"
pngs_folder = "../assets"

combinations = {
    "n": (4, -90),
    "e": (4, 180),
    "s": (4, 90),
    "w": (4, 0),
    "nw": (3, 0),
    "ne": (3, -90),
    "es": (3, 180),
    "sw": (3, 90),
    "nes": (2, 180),
    "esw": (2, 90),
    "nws": (2, 0),
    "nwe": (2, -90),
    "nesw": (1, 0),
}

def fix_rgba_image(image_path):
    image = Image.open(image_path)
    
    if image.mode != 'RGBA':
        print(f"Converting {image_path} to RGBA")
        image = image.convert("RGBA")        
        image.save(image_path)
    else:
        r, g, b, a = image.split()
        if a.getextrema() == (255, 255):
            print(f"No transparency detected in {image_path}. Fixing alpha channel.")
            image = Image.merge("RGBA", (r, g, b, a))
            image.save(image_path)

def export_all_tiles(aseprite_assets, destination_folder):
    print("Generating tiles...")
    os.system("rm -rf temp")
    os.system("mkdir temp")

    os.system(f"{aseprite_path} -b {aseprite_assets}/bg_tiles.aseprite --save-as temp/bg_tiles-0.png")
        
    for file in os.listdir("temp"):
        if file.endswith(".png"):
            fix_rgba_image(f"temp/{file}")    

    indexed_biomes = [("water", 0), ("desert", 1), ("grass", 2), ("rock", 3), ("snow", 4)]

    for frame in range(0, 4):
        tiles = Image.open(f"temp/bg_tiles-{frame}.png")

        for base_biome, row in indexed_biomes:
            for border_biome, other_row in indexed_biomes:
                if row == other_row: 
                    y = tile_size * row
                    result = tiles.crop((0, y, tile_size, y + tile_size))
                    result.save(f"{destination_folder}/bg_tile_{base_biome}-{frame}.png")       
                else:
                    for borders in combinations.keys():
                        column, rotation = combinations[borders]

                        y = tile_size * row
                        result = tiles.crop((0, y, tile_size, y + tile_size))

                        x = tile_size * column
                        y = tile_size * other_row
                        new_layer = tiles.crop((x, y, x + tile_size, y + tile_size))
                        new_layer = new_layer.rotate(rotation, expand=False)

                        result.paste(new_layer, (0, 0), new_layer)
                        result.save(f"{destination_folder}/bg_tile_{base_biome}_{border_biome}_{borders}-{frame}.png")        

os.system(f"rm -rf {pngs_folder}/bg_tile*")
export_all_tiles(aseprite_assets, pngs_folder)