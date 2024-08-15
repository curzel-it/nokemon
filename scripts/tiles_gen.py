import pdb
from PIL import Image
import sys
import os
import subprocess

aseprite_path = "/Applications/Aseprite.app/Contents/MacOS/aseprite"
biomes = "grass desert rock snow water".split(" ")
aseprite_assets = "../aseprite"
pngs_folder = "../assets"

combinations = {
    "n": ("w", -90),
    "e": ("w", 180),
    "s": ("w", 90),
    "w": ("w", 0),
    "nw": ("nw", 0),
    "ne": ("nw", -90),
    "es": ("nw", 180),
    "sw": ("nw", 90),
    "nes": ("nws", 180),
    "wes": ("nws", 90),
    "nws": ("nws", 0),
    "nwe": ("nws", -90),
    "news": ("news", 0),
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

    for biome in biomes:
        os.system(f"{aseprite_path} -b {aseprite_assets}/bg_tile_{biome}.aseprite --save-as {destination_folder}/bg_tile_{biome}-0.png")
        os.system(f"{aseprite_path} -b {aseprite_assets}/bg_tile_{biome}.aseprite --save-as temp/{biome}_base-0.png")
        os.system(f"{aseprite_path} -b {aseprite_assets}/bg_tile_{biome}_border_w.aseprite --save-as temp/{biome}_w-0.png")
        os.system(f"{aseprite_path} -b {aseprite_assets}/bg_tile_{biome}_border_nw.aseprite --save-as temp/{biome}_nw-0.png")
        os.system(f"{aseprite_path} -b {aseprite_assets}/bg_tile_{biome}_border_nws.aseprite --save-as temp/{biome}_nws-0.png")
        os.system(f"{aseprite_path} -b {aseprite_assets}/bg_tile_{biome}_border_news.aseprite --save-as temp/{biome}_news-0.png")
        
    for file in os.listdir("temp"):
        if file.endswith(".png"):
            fix_rgba_image(f"temp/{file}")

    for base_biome in biomes:
        for border_biome in biomes:
            if base_biome == border_biome: continue

            for borders in combinations.keys():
                for frame in range(0, 4):
                    layer_name, rotation = combinations[borders]
                    base_path = f"temp/{base_biome}_base-{frame}.png"
                    result = Image.open(base_path)

                    new_layer_path = f"temp/{border_biome}_{layer_name}-{frame}.png"
                    new_layer = Image.open(new_layer_path)
                    new_layer = new_layer.rotate(rotation, expand=False)

                    result.paste(new_layer, (0, 0), new_layer)
                    result.save(f"{destination_folder}/bg_tile_{base_biome}_{border_biome}_{borders}-{frame}.png")        

os.system(f"rm -rf {pngs_folder}/bg_tile*")
export_all_tiles(aseprite_assets, pngs_folder)