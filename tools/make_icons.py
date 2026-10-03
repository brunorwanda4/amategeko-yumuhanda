"""Generate every app icon from the master logo.

Input:  assets/branding/logo_source.png  (black rounded square, white road,
        on a white page background)

Output:
  assets/branding/logo.png                         1024x1024, transparent outside the square
  crates/desktop/resources/app.ico                 Windows exe/window icon (16..256 px)
  crates/mobile/android/app/src/main/res/
      mipmap-*/ic_launcher.png, ic_launcher_round.png   legacy launcher icons
      drawable-*/ic_launcher_foreground.png             adaptive-icon foreground (white road)

Run once after changing the logo:  python tools/make_icons.py
Needs Pillow (already in tools/requirements.txt).
"""

from __future__ import annotations

from collections import deque
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "assets" / "branding" / "logo_source.png"
LOGO_OUT = ROOT / "assets" / "branding" / "logo.png"
ICO_OUT = ROOT / "crates" / "desktop" / "resources" / "app.ico"
RES = ROOT / "crates" / "mobile" / "android" / "app" / "src" / "main" / "res"

# Pixels lighter than this that touch the page border are "page background".
LIGHT = 128

LEGACY_SIZES = {"mdpi": 48, "hdpi": 72, "xhdpi": 96, "xxhdpi": 144, "xxxhdpi": 192}
# Adaptive icons are 108dp; 1dp = 1px at mdpi.
ADAPTIVE_SIZES = {"mdpi": 108, "hdpi": 162, "xhdpi": 216, "xxhdpi": 324, "xxxhdpi": 432}
# The road must stay inside the 66dp safe zone of the 108dp canvas.
FOREGROUND_FRACTION = 58 / 108


def outer_mask(gray: Image.Image) -> list[bool]:
    """Flood-fill from the border: True for light pixels connected to the edge."""
    w, h = gray.size
    px = gray.load()
    seen = [False] * (w * h)
    q: deque[tuple[int, int]] = deque()
    for x in range(w):
        q.extend([(x, 0), (x, h - 1)])
    for y in range(h):
        q.extend([(0, y), (w - 1, y)])
    while q:
        x, y = q.popleft()
        i = y * w + x
        if seen[i] or px[x, y] <= LIGHT:
            continue
        seen[i] = True
        if x > 0:
            q.append((x - 1, y))
        if x < w - 1:
            q.append((x + 1, y))
        if y > 0:
            q.append((x, y - 1))
        if y < h - 1:
            q.append((x, y + 1))
    return seen


def split_layers(src: Image.Image) -> tuple[Image.Image, Image.Image]:
    """Return (full icon on transparent page, white road on transparent)."""
    gray = src.convert("L")
    w, h = gray.size
    outer = outer_mask(gray)
    g = gray.load()

    full = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    road = Image.new("RGBA", (w, h), (255, 255, 255, 0))
    fp, rp = full.load(), road.load()
    for y in range(h):
        for x in range(w):
            lum = g[x, y]
            if outer[y * w + x]:
                # Anti-aliased edge of the square: keep its darkness as alpha.
                fp[x, y] = (0, 0, 0, 255 - lum)
            else:
                fp[x, y] = (lum, lum, lum, 255)
                rp[x, y] = (255, 255, 255, lum)
    return full, road


def square_crop(img: Image.Image, bbox: tuple[int, int, int, int], pad: float) -> Image.Image:
    """Crop to bbox, centre it on a transparent square with `pad` margin per side."""
    part = img.crop(bbox)
    side = max(part.size)
    canvas = int(round(side * (1 + 2 * pad)))
    out = Image.new("RGBA", (canvas, canvas), (0, 0, 0, 0))
    out.paste(part, ((canvas - part.width) // 2, (canvas - part.height) // 2), part)
    return out


def save_png(img: Image.Image, path: Path, size: int) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    img.resize((size, size), Image.LANCZOS).save(path, optimize=True)


def round_icon(full: Image.Image, size: int) -> Image.Image:
    """Legacy round icon: the black square clipped to a circle."""
    icon = full.resize((size, size), Image.LANCZOS)
    big = size * 4
    mask = Image.new("L", (big, big), 0)
    ImageDraw.Draw(mask).ellipse((0, 0, big - 1, big - 1), fill=255)
    mask = mask.resize((size, size), Image.LANCZOS)
    out = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    out.paste(icon, (0, 0), mask)
    return out


def main() -> None:
    raw = Image.open(SRC).convert("RGBA")
    # Transparent page pixels may hide black RGB; flatten onto white first.
    src = Image.new("RGBA", raw.size, (255, 255, 255, 255))
    src.alpha_composite(raw)
    full, road = split_layers(src)

    square_bbox = full.getbbox()
    road_bbox = road.getbbox()
    if square_bbox is None or road_bbox is None:
        raise SystemExit("logo_source.png does not look like the expected logo")

    logo = square_crop(full, square_bbox, pad=0.0)
    LOGO_OUT.parent.mkdir(parents=True, exist_ok=True)
    logo.resize((1024, 1024), Image.LANCZOS).save(LOGO_OUT, optimize=True)

    ICO_OUT.parent.mkdir(parents=True, exist_ok=True)
    logo.resize((256, 256), Image.LANCZOS).save(
        ICO_OUT, sizes=[(s, s) for s in (16, 24, 32, 48, 64, 128, 256)]
    )

    # Legacy launcher icons: small margin like the Material legacy grid.
    legacy = square_crop(full, square_bbox, pad=0.04)
    for dpi, size in LEGACY_SIZES.items():
        save_png(legacy, RES / f"mipmap-{dpi}" / "ic_launcher.png", size)
        round_icon(full.crop(square_bbox), size).save(
            RES / f"mipmap-{dpi}" / "ic_launcher_round.png", optimize=True
        )

    # Adaptive foreground: road only, centred in the safe zone.
    road_sq = square_crop(road, road_bbox, pad=0.0)
    pad = (1 / FOREGROUND_FRACTION - 1) / 2
    fg = square_crop(road_sq, (0, 0, road_sq.width, road_sq.height), pad=pad)
    for dpi, size in ADAPTIVE_SIZES.items():
        save_png(fg, RES / f"drawable-{dpi}" / "ic_launcher_foreground.png", size)

    print("icons written")


if __name__ == "__main__":
    main()
