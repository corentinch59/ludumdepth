from PIL import Image

# SPLIT GIFS

gif_paths = [ "./raw/player.gif" ]

for gif_path in gif_paths:
    name = gif_path.split("/")[-1][:-4]
    out = f"./assets/textures/{name}.png"
    gif = Image.open(gif_path)
    frames = []

    try:
        while True:
            frames.append(gif.copy())
            gif.seek(gif.tell() + 1)
    except EOFError:
        pass

    columns = 5
    rows = (len(frames) + columns - 1) // columns

    max_width = max(frame.width for frame in frames)
    max_height = max(frame.height for frame in frames)

    atlas_width = columns * max_width
    atlas_height = rows * max_height
    atlas = Image.new("RGBA", (atlas_width, atlas_height))

    for index, frame in enumerate(frames):
        row = index // columns
        column = index % columns
        atlas.paste(frame, (column * max_width, row * max_height))

    atlas.save(out)
