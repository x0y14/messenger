import nanoid
from PIL import Image


def generate_id():
    return nanoid.generate(size=30)


def process_image(sub_type, file_binary) -> str:
    fid = generate_id()

    if sub_type == "gif":
        im = Image.open(file_binary)
        im.save(f"storage/{fid}", format="GIF", save_all=True)
        thumbnail_crop(fid, im)
        im.close()
    else:
        im = Image.open(file_binary)
        im.save(f"storage/{fid}", format="PNG")
        thumbnail_crop(fid, im)
        im.close()

    return fid


def thumbnail_crop(fid, img):
    size = (250, 250)
    img.thumbnail(size, Image.ANTIALIAS)
    img_width, img_height = img.size
    background = Image.new("RGBA", (250, 250), (0, 0, 0, 255))
    background_width, background_height = background.size

    offset = ((background_width - img_width) // 2, (background_height - img_height) // 2)

    background.paste(img, offset)
    background.save(f"storage/250x250/{fid}", format="PNG")

    img.close()
    background.close()


