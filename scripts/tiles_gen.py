import numpy as np
from PIL import Image

width, height = 15, 15

def noisy_image_data(r, g, b):
    image_data = np.zeros((height, width, 4), dtype=np.uint8)

    for y in range(height):
        for x in range(width):
            intensity = 0.6 + np.random.rand() * 0.4
            image_data[y, x] = [r * intensity, g * intensity, b * intensity, 255]
    return image_data

for i in range(0, 10):
    image_data = noisy_image_data(20, 225, 80)
    image = Image.fromarray(image_data, 'RGBA')
    image.save(f'../assets/bg_tile_grass-{i}.png')

for i in range(0, 10):
    image_data = noisy_image_data(50, 150, 225)
    image = Image.fromarray(image_data, 'RGBA')
    image.save(f'../assets/bg_tile_water-{i}.png')

for i in range(0, 10):
    image_data = noisy_image_data(230, 230, 150)
    image = Image.fromarray(image_data, 'RGBA')
    image.save(f'../assets/bg_tile_desert-{i}.png')

for i in range(0, 10):
    image_data = noisy_image_data(150, 150, 150)
    image = Image.fromarray(image_data, 'RGBA')
    image.save(f'../assets/bg_tile_rock-{i}.png')

for i in range(0, 10):
    image_data = noisy_image_data(255, 255, 255)
    image = Image.fromarray(image_data, 'RGBA')
    image.save(f'../assets/bg_tile_snow-{i}.png')
