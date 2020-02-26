"""Automatically build the tileset.ron file"""

def build(x: int, y: int, width: int, height: int):
    """x and y are the numbers of tiles, and width and height the number of pixels in the image"""
    tile_width = height // x
    tile_height = width // y
    data = f"""(
        texture_width: {width},
        texture_height: {height},
        sprites: [
    """
    sprite_template = """(
        x: {x},
        y: {y},
        width: {width},
        height: {height},
    ),
    """
    for i in range(y):
        for j in range(x):
            data += sprite_template.format(x=j * tile_height, y=i * tile_width, width=tile_width, height=tile_height)
    data += "    ],\n"
    data += ")"
    with open("assets/tiles/tileset.ron", mode="w") as f:
        f.write(data)

if __name__ == "__main__":
    from sys import argv
    if len(argv) >= 5:
        build(*argv[1:4])
    else:
        build(20, 20, 400, 400)