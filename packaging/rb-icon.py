#!/usr/bin/env python3
"""Draw the RB Deck app icon and write every size the packages use.

The icon is original work for RB Deck (MIT OR Apache-2.0): a navy tile, a white slide with an
amber title bar and three rising teal bars. The same shapes are written as the scalable SVG, and
crates/ui-egui/src/dialogs/about.rs paints them in code (paint_logo).

    python3 packaging/rb-icon.py      (needs Pillow)
"""

from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "assets" / "app-icon"
APP_ID = "ai.storyteller.deckcraft"  # file names the packaging scripts expect; renamed with the app id
NAVY, SLIDE, AMBER, TEAL = "#1d2b4f", "#f7f8fa", "#f5b301", "#2bb5a3"
# (kind, x, y, w, h, radius, colour) on a 512 grid
SHAPES = [
    ("rect", 16, 16, 480, 480, 112, NAVY),
    ("rect", 80, 112, 352, 232, 20, SLIDE),
    ("rect", 112, 144, 150, 22, 6, AMBER),
    ("rect", 128, 262, 52, 54, 6, TEAL),
    ("rect", 206, 226, 52, 90, 6, TEAL),
    ("rect", 284, 186, 52, 130, 6, TEAL),
    ("rect", 236, 344, 40, 52, 0, SLIDE),
    ("rect", 176, 392, 160, 24, 12, SLIDE),
]


def draw(size: int) -> Image.Image:
    big = 1024
    k = big / 512
    image = Image.new("RGBA", (big, big), (0, 0, 0, 0))
    pen = ImageDraw.Draw(image)
    for _, x, y, w, h, r, colour in SHAPES:
        pen.rounded_rectangle([x * k, y * k, (x + w) * k - 1, (y + h) * k - 1], radius=r * k, fill=colour)
    return image if size == big else image.resize((size, size), Image.LANCZOS)


def svg() -> str:
    parts = [f'<rect x="{x}" y="{y}" width="{w}" height="{h}"{f" rx={chr(34)}{r}{chr(34)}" if r else ""} fill="{c}"/>'
             for _, x, y, w, h, r, c in SHAPES]
    return f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">{"".join(parts)}</svg>\n'


def main() -> None:
    draw(1024).save(OUT / "deckcraft-1024.png")
    for size in (16, 24, 32, 48, 64, 128, 256, 512):
        folder = OUT / "hicolor" / f"{size}x{size}" / "apps"
        folder.mkdir(parents=True, exist_ok=True)
        draw(size).save(folder / f"{APP_ID}.png")
    (OUT / "hicolor" / "scalable" / "apps" / f"{APP_ID}.svg").write_text(svg())
    draw(256).save(OUT / "deckcraft.ico", sizes=[(s, s) for s in (16, 24, 32, 48, 64, 128, 256)])
    draw(1024).save(OUT / "deckcraft.icns")


if __name__ == "__main__":
    main()
