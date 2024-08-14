from PIL import Image
import sys
import os
import subprocess

aseprite_path = "/Applications/Aseprite.app/Contents/MacOS/aseprite"
aseprite_assets = "../aseprite"
pngs_folder = "../assets"
directions = "n e s w".split(" ")
directions_8 = "n e s w ne es sw nw".split(" ")
walk_layers = [f"walk{d}" for d in directions]
still_layers = [f"still{d}" for d in directions]
biomes = "desert water rock snow grass".split(" ")

def export_aseprite(file_path, destination_folder):
    filename = file_path.split("/")[-1]
    if filename == "palette.aseprite": return
    elif filename == "world.aseprite": export_level(file_path, destination_folder)
    elif filename.startswith("bg_tile_"): export_bg_tile(file_path, destination_folder)
    else: export_character(file_path, destination_folder)

def export_level(file_path, destination_folder):
    cmd = f"{aseprite_path} -b {file_path} --layer biome --save-as {destination_folder}/../levels/world_biome.png"
    os.system(cmd)
    cmd = f"{aseprite_path} -b {file_path} --layer constructions --save-as {destination_folder}/../levels/world_constructions.png"
    os.system(cmd)

def list_layers(path):
    command = [aseprite_path, "-b", "--list-layers", path]
    result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    layers = result.stdout.strip().splitlines()    
    return layers

def export_bg_tile(file_path, destination_folder):
    asset_name = asset_name_from_file_path(file_path)
    layers = list_layers(file_path)

    base_asset_no_index = f"{destination_folder}/{asset_name}"
    base_asset = f"{base_asset_no_index}-0.png"
    cmd = f"{aseprite_path} -b {file_path} --layer base --save-as {base_asset}"
    os.system(cmd)

    for biome in biomes:
        if f"{biome}_nw" in layers:
            regular = f"{destination_folder}/{asset_name}_{biome}-0.png"
            cmd = f"{aseprite_path} -b {file_path} --layer base --save-as {regular}"
            os.system(cmd)

            west = f"temp/{asset_name}_{biome}_w-0.png"
            cmd = f"{aseprite_path} -b {file_path} --layer {biome}_w --save-as {west}"
            os.system(cmd)

            north_west = f"temp/{asset_name}_{biome}_nw-0.png"
            cmd = f"{aseprite_path} -b {file_path} --layer {biome}_nw --save-as {north_west}"
            os.system(cmd)

            merge_sprites_layers(regular, west, "_w-", "_n-", -90, 4)
            merge_sprites_layers(regular, west, "_w-", "_e-", 180, 4)
            merge_sprites_layers(regular, west, "_w-", "_s-", 90, 4)
            merge_sprites_layers(regular, west, "_w-", "_w-", 0, 4)
            merge_sprites_layers(regular, north_west, "_nw-", "_nw-", 0, 4)
            merge_sprites_layers(regular, north_west, "_nw-", "_ne-", -90, 4)
            merge_sprites_layers(regular, north_west, "_nw-", "_es-", 180, 4)
            merge_sprites_layers(regular, north_west, "_nw-", "_sw-", 90, 4)

def merge_sprites_layers(first_layer_path, second_layer_path, original_suffix, replace_suffix, second_layer_rotation, number_of_frames):
    destination_folder = "/".join(first_layer_path.split("/")[:-1])

    for frame in range(0, number_of_frames):
        merged_destination = second_layer_path
        merged_destination = merged_destination.replace("temp/", f"{destination_folder}/")
        merged_destination = merged_destination.replace(original_suffix, replace_suffix)
        merged_destination = merged_destination.replace("-0.", f"-{frame}.")

        second_layer = Image.open(second_layer_path.replace("-0.", f"-{frame}."))        
        second_layer = second_layer.rotate(second_layer_rotation, expand=True)

        try:
            first_layer = Image.open(first_layer_path.replace("-0.", f"-{frame}."))        
            first_layer.paste(second_layer, (0, 0), second_layer)        
            first_layer.save(merged_destination)        
        except Exception as e:
            print(e)
            print(first_layer_path)
            print(second_layer_path)
            print(merged_destination)


def export_character(file_path, destination_folder):
    asset_name = asset_name_from_file_path(file_path)
    layers = list_layers(file_path)

    non_still_non_movement_layers = layers
    non_still_non_movement_layers = [l for l in layers if not "still" in l]
    non_still_non_movement_layers = [l for l in layers if not "walk" in l]

    if "walk" in layers:
        for direction in directions:
            cmd = f"{aseprite_path} -b {file_path} --layer walk --save-as {destination_folder}/{asset_name}_walk{direction}-0.png"
            os.system(cmd)

    if "walkn" in layers:
        for direction in directions:
            cmd = f"{aseprite_path} -b {file_path} --layer walk{direction} --save-as {destination_folder}/{asset_name}_walk{direction}-0.png"
            os.system(cmd)

    if "still" in layers:
        for direction in directions:
            cmd = f"{aseprite_path} -b {file_path} --layer still --save-as {destination_folder}/{asset_name}_still{direction}-0.png"
            os.system(cmd)

    if "stilln" in layers:
        for direction in directions:
            cmd = f"{aseprite_path} -b {file_path} --layer still{direction} --save-as {destination_folder}/{asset_name}_still{direction}-0.png"
            os.system(cmd)

    if ("walk" in layers or "walkn" in layers) and "still" not in layers and "stilln" not in layers:
        for direction in directions:
            cmd = f"cp {destination_folder}/{asset_name}_walk{direction}-0.png {destination_folder}/{asset_name}_still{direction}-0.png"
            os.system(cmd)

    for layer in non_still_non_movement_layers:
        cmd = f"{aseprite_path} -b {file_path} --layer {layer} --save-as {destination_folder}/{asset_name}_{layer}-0.png"
        os.system(cmd)

def asset_name_from_file_path(file_path):
    asset_name = file_path.split("/")[-1].split(".")[0]
    asset_name = asset_name[:-1] if asset_name.endswith("-") else asset_name
    return asset_name

def find_aseprite_files(folder, tag):
    paths = []
    for root, _, files in os.walk(folder):
        for file in files:
            if tag in file.lower() and (file.endswith(".aseprite") or file.endswith(".ase")):
                paths.append(os.path.join(root, file))
    return paths


def export_all_aseprite(tag, root_folder, destination_folder):
    print(f"Looking for *.aseprite and *.ase file in {root_folder}...")
    if tag != "":
        print(f"Also filtering by `{tag}`")
    files = find_aseprite_files(root_folder, tag)
    print(f"Found {len(files)} files")
    for i, file in enumerate(files):
        print(f"Exporting file {i+1} out of {len(files)}")
        export_aseprite(file, destination_folder)
    print(f"All done!")

os.system("mkdir temp")
tag = sys.argv[-1] if len(sys.argv) == 2 else ""
# os.system(f"rm -rf {pngs_folder}/*")
export_all_aseprite(tag, aseprite_assets, pngs_folder)
os.system("rm -rf temp")