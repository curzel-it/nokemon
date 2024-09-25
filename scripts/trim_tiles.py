import json

def trim_tiles(tiles, new_rows, new_cols):
    original_rows = len(tiles)
    original_cols = len(tiles[0]) if tiles else 0

    if new_rows > original_rows or new_cols > original_cols:
        raise ValueError("New dimensions must be less than or equal to the original dimensions.")

    # Calculate rows to remove
    rows_to_remove = original_rows - new_rows
    top_rows = rows_to_remove // 2
    bottom_rows = rows_to_remove - top_rows

    # Trim rows
    trimmed_tiles = tiles[top_rows : original_rows - bottom_rows]

    # Calculate columns to remove
    cols_to_remove = original_cols - new_cols
    left_cols = cols_to_remove // 2
    right_cols = cols_to_remove - left_cols

    # Trim columns in each row
    for i in range(len(trimmed_tiles)):
        trimmed_tiles[i] = trimmed_tiles[i][left_cols : original_cols - right_cols]

    return trimmed_tiles

def main():
    # Set the new dimensions
    new_rows = 80
    new_cols = 120

    # Load the JSON data from a file
    with open('../levels/1002.json', 'r') as f:
        data = json.load(f)

    # Process "biome_tiles"
    biome_tiles = data.get("biome_tiles", {})
    if "tiles" in biome_tiles:
        biome_tiles["tiles"] = trim_tiles(biome_tiles["tiles"], new_rows, new_cols)

    # Process "constructions_tiles"
    constructions_tiles = data.get("constructions_tiles", {})
    if "tiles" in constructions_tiles:
        constructions_tiles["tiles"] = trim_tiles(constructions_tiles["tiles"], new_rows, new_cols)

    # Save the modified JSON data to a new file
    with open('trimmed_world_data.json', 'w') as f:
        json.dump(data, f, indent=2)

    print("Tiles have been trimmed and saved to 'trimmed_world_data.json'.")

if __name__ == "__main__":
    main()
