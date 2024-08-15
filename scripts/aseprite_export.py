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
    elif filename.startswith("bg_tile"): return
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

def export_character(file_path, destination_folder):
    asset_name = asset_name_from_file_path(file_path)
    cmd = f"{aseprite_path} -b --split-layers {file_path} --sheet-type rows --sheet {destination_folder}/{asset_name}.png"
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
export_all_aseprite(tag, aseprite_assets, pngs_folder)
