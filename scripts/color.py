import numpy as np
from PIL import Image

file = "assets/tiles/tileset.png"
im = Image.open(file).convert(mode="RGBA")

for x in range(im.width):
    for y in range(im.height):
        if im.getpixel((x,y)) == (46, 196, 24, 255):
            im.putpixel((x, y), (0, 0, 0, 0))

im.save(file)