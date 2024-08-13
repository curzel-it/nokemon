import sys
import os
import subprocess

aseprite_path = "/Applications/Aseprite.app/Contents/MacOS/aseprite"
aseprite_assets = "../aseprite"
pngs_folder = "../assets"
directions = "n e s w".split(" ")
walk_layers = [f"walk{d}" for d in directions]
still_layers = [f"still{d}" for d in directions]

def export_aseprite(file_path, destination_folder):
    filename = file_path.split("/")[-1]
    if filename == "palette.aseprite": return
    elif filename == "world.aseprite": export_aseprite_level(file_path, destination_folder)
    else: export_aseprite_character(file_path, destination_folder)

def export_aseprite_level(file_path, destination_folder):
    cmd = f"{aseprite_path} -b {file_path} --layer biome --save-as {destination_folder}/../levels/world_biome.png"
    os.system(cmd)
    cmd = f"{aseprite_path} -b {file_path} --layer constructions --save-as {destination_folder}/../levels/world_constructions.png"
    os.system(cmd)

def list_layers(path):
    command = [
        aseprite_path, 
        "-b", 
        "--list-layers", 
        path
    ]
    
    # Run the command and capture the output
    result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    
    # Split the output into lines and strip any extra whitespace
    layers = result.stdout.strip().splitlines()
    
    return layers

def export_aseprite_character(file_path, destination_folder):
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


tag = sys.argv[-1] if len(sys.argv) == 2 else ""
# os.system(f"rm -rf {pngs_folder}/*")
export_all_aseprite(tag, aseprite_assets, pngs_folder)