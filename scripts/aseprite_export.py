import os

aseprite_path = "/Applications/Aseprite.app/Contents/MacOS/aseprite"
aseprite_assets = "../aseprite"
pngs_folder = "../assets"

def export_aseprite(file_path, destination_folder):
    if "/palettes" in file_path:
        return
    elif "/characters" in file_path:
        export_aseprite_character(file_path, destination_folder)
    else:
        export_aseprite_default(file_path, destination_folder)

def export_aseprite_default(file_path, destination_folder):
    asset_name = file_path.split("/")[-1].split(".")[0]
    asset_name = asset_name[:-1] if asset_name.endswith("-") else asset_name
    ignore_layers = '--ignore-layer "Talking" --ignore-layer "talking"'
    cmd = f"{aseprite_path} -b {file_path} {ignore_layers} --save-as {destination_folder}/{asset_name}-0.png"
    os.system(cmd)

def export_aseprite_character(file_path, destination_folder):
    asset_name = file_path.split("/")[-1].split(".")[0]
    asset_name = asset_name[:-1] if asset_name.endswith("-") else asset_name

    directions = "n ne e se s so o no".split(" ")
    layers = [f"walk_{d}" for d in directions]
    
    for layer in layers:
        cmd = f"{aseprite_path} -b {file_path} --layer {layer} --save-as {destination_folder}/{asset_name}_{layer}-0.png"
        os.system(cmd)

def find_aseprite_files(folder):
    paths = []
    for root, dirs, files in os.walk(folder):
        for file in files:
            if file.endswith(".aseprite") or file.endswith(".ase"):
                paths.append(os.path.join(root, file))
    return paths


def export_all_aseprite(root_folder, destination_folder):
    print(f"Looking for *.aseprite and *.ase file in {root_folder}...")
    files = find_aseprite_files(root_folder)
    print(f"Found {len(files)} files")
    for i, file in enumerate(files):
        if i % 10 == 0:
            print(f"Exported {i} files out of {len(files)}")
        export_aseprite(file, destination_folder)


os.system(f"rm -rf {pngs_folder}")
export_all_aseprite(aseprite_assets, pngs_folder)