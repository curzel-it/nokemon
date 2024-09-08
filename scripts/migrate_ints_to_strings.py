import json
import os


def convert_tile_data(tiles):
    """Convert the tile data from integers to strings using the provided mapping."""
    return [''.join([str(tile) for tile in row]) for row in tiles]

def convert_world_json(old_json):
    """Convert the old world JSON format to the new format."""
    if 'biome_tiles' in old_json:
        old_json['biome_tiles']['tiles'] = convert_tile_data(old_json['biome_tiles']['tiles'])
    if 'constructions_tiles' in old_json:
        old_json['constructions_tiles']['tiles'] = convert_tile_data(old_json['constructions_tiles']['tiles'])
    return old_json

def convert_json_file(input_path, output_path):
    """Convert a single JSON file from the old format to the new format."""
    # Read the old JSON file
    with open(input_path, 'r') as infile:
        old_json = json.load(infile)
    
    # Convert the JSON structure
    new_json = convert_world_json(old_json)
    
    # Write the new JSON file
    with open(output_path, 'w') as outfile:
        json.dump(new_json, outfile, indent=4)
    print(f"Converted {input_path} and saved to {output_path}")

def batch_convert_json_files(input_dir, output_dir):
    """Convert all JSON files in the input directory and save to the output directory."""
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    for filename in os.listdir(input_dir):
        if filename.endswith(".json"):
            input_path = os.path.join(input_dir, filename)
            output_path = os.path.join(output_dir, filename)
            convert_json_file(input_path, output_path)

# Example usage
if __name__ == "__main__":
    input_directory = "../old-levels"  # Directory containing old JSON files
    output_directory = "../levels"  # Directory to save converted JSON files
    batch_convert_json_files(input_directory, output_directory)
