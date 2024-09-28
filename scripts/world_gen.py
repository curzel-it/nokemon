import json
import random
import argparse
from noise import pnoise2

WIDTH = 120
HEIGHT = 80

# Parameters for the noise function
freq = 0.8  # Lower frequency for larger features
octaves = 3  # Reduced octaves for smoother noise
persistence = 0.5
lacunarity = 2.0

# Tile representation
WATER = '2'
SAND = '4'
GRASS = '1'

# Initialize the map with water
tiles = [[WATER for _ in range(WIDTH)] for _ in range(HEIGHT)]

# Adjusted falloff function for smoother edges
def falloff(x, y):
    nx = x / WIDTH * 2 - 1
    ny = y / HEIGHT * 2 - 1
    distance = ((nx**2 + ny**2) ** 0.5) / (2 ** 0.5)  # Normalize distance
    return distance ** 2  # Less severe falloff

for y in range(HEIGHT):
    for x in range(WIDTH):
        # Borders remain water
        if x == 0 or y == 0 or x == WIDTH - 1 or y == HEIGHT - 1:
            continue

        nx = x / WIDTH - 0.5
        ny = y / HEIGHT - 0.5

        # Generate noise value
        e = pnoise2(
            nx * freq,
            ny * freq,
            octaves=octaves,
            persistence=persistence,
            lacunarity=lacunarity,
            repeatx=WIDTH,
            repeaty=HEIGHT,
            base=random.randint(0, 1000)  # Fixed base for consistency
        )

        # Apply the falloff function and adjust to increase landmass
        e = e - falloff(x, y) + 0.6  # Adjust constant for island size

        # Assign biome based on elevation value
        if e > 0.05:  # Sand zone
            tiles[y][x] = SAND
        else:
            tiles[y][x] = WATER

# Function to smooth the map edges
def smooth_map(tiles):
    new_tiles = [row[:] for row in tiles]
    for y in range(1, HEIGHT - 1):
        for x in range(1, WIDTH - 1):
            counts = {WATER: 0, SAND: 0, GRASS: 0}
            for dy in [-1, 0, 1]:
                for dx in [-1, 0, 1]:
                    if dy == 0 and dx == 0:
                        continue
                    neighbor = tiles[y + dy][x + dx]
                    counts[neighbor] += 1
            max_tile = max(counts, key=counts.get)
            new_tiles[y][x] = max_tile
    return new_tiles

# Apply smoothing to reduce jagged edges
for _ in range(3):
    tiles = smooth_map(tiles)

# Function to remove small inland water bodies (puddles)
def remove_inland_water(tiles):
    visited = [[False for _ in range(WIDTH)] for _ in range(HEIGHT)]
    queue = []

    # Enqueue all water tiles on the borders
    for x in range(WIDTH):
        if tiles[0][x] == WATER:
            queue.append((0, x))
            visited[0][x] = True
        if tiles[HEIGHT - 1][x] == WATER:
            queue.append((HEIGHT - 1, x))
            visited[HEIGHT - 1][x] = True

    for y in range(HEIGHT):
        if tiles[y][0] == WATER:
            queue.append((y, 0))
            visited[y][0] = True
        if tiles[y][WIDTH - 1] == WATER:
            queue.append((y, WIDTH - 1))
            visited[y][WIDTH - 1] = True

    # Perform BFS to mark all water tiles connected to the borders
    while queue:
        y, x = queue.pop(0)
        for dy, dx in [(-1,0),(1,0),(0,-1),(0,1)]:
            ny, nx = y + dy, x + dx
            if 0 <= ny < HEIGHT and 0 <= nx < WIDTH:
                if not visited[ny][nx] and tiles[ny][nx] == WATER:
                    visited[ny][nx] = True
                    queue.append((ny, nx))

    # Any water tile not visited is inland water; convert to SAND
    for y in range(HEIGHT):
        for x in range(WIDTH):
            if tiles[y][x] == WATER and not visited[y][x]:
                tiles[y][x] = SAND

    return tiles

# Remove inland water bodies
tiles = remove_inland_water(tiles)

# Helper function to safely get tile value, assuming out-of-bounds tiles are WATER
def get_tile(tiles, y, x):
    if 0 <= y < HEIGHT and 0 <= x < WIDTH:
        return tiles[y][x]
    else:
        return WATER

# Now, add grass to the interior of the island, including edges
def add_grass(tiles):
    new_tiles = [row[:] for row in tiles]
    for y in range(HEIGHT):
        for x in range(WIDTH):
            if get_tile(tiles, y, x) != WATER and \
                get_tile(tiles, y-1, x) != WATER and \
                get_tile(tiles, y-2, x) != WATER and \
                get_tile(tiles, y+1, x) != WATER and \
                get_tile(tiles, y+2, x) != WATER and \
                get_tile(tiles, y, x-1) != WATER and \
                get_tile(tiles, y, x-2) != WATER and \
                get_tile(tiles, y, x+1) != WATER and \
                get_tile(tiles, y, x+2) != WATER and \
                get_tile(tiles, y+1, x+1) != WATER and \
                get_tile(tiles, y-1, x+1) != WATER and \
                get_tile(tiles, y-1, x-1) != WATER and \
                get_tile(tiles, y+1, x-1) != WATER and \
                get_tile(tiles, y-2, x-1) != WATER and \
                get_tile(tiles, y+2, x-1) != WATER and \
                get_tile(tiles, y-1, x-2) != WATER and \
                get_tile(tiles, y-1, x+2) != WATER:
                    new_tiles[y][x] = GRASS
    return new_tiles

# Apply the grass function after the rest of the map is generated
tiles = add_grass(tiles)

# Convert each row to a string
tile_strings = [''.join(row) for row in tiles]

# Set up command-line argument parsing
parser = argparse.ArgumentParser(description='Generate a biome map with a specific world ID.')
parser.add_argument('world_id', type=int, help='The ID of the world to be generated.')

# Parse the arguments
args = parser.parse_args()

# Construct the JSON object
world_data = {
    "id": args.world_id,
    "biome_tiles": {
        "tiles": tile_strings,
        "sheet_id": 1002
    },
    "constructions_tiles": {
        "tiles": ["0" * WIDTH] * HEIGHT,
        "sheet_id": 1003
    },
    "entities": [],
    "creep_spawn_enabled": False,
    "creep_spawn_interval": 0.0,
    "is_interior": False
}

# Output the JSON to the file with the world ID in the name
output_filename = f"/Users/curzel/dev/nokemon/levels/{args.world_id}.json"
with open(output_filename, "w") as f:
    f.write(json.dumps(world_data, indent=2))

print(f"World {args.world_id} has been generated and saved to {output_filename}")
