#!/usr/bin/env python3
"""Convert P3 PPM renders to PNG previews under `scenes/pngs/`.

Stdlib only (`struct` + `zlib`) — no Pillow, no `image` crate, consistent with
`scripts/gen-demo-textures.py`. The `.ppm` stays the required audit
deliverable; the `.png` is only a convenience copy so renders can be viewed
without a PPM-aware tool. Never deletes or modifies the source `.ppm`.

Usage:

    python3 scripts/ppm_to_png.py
        # every complete scenes/*.ppm → scenes/pngs/<stem>.png

    python3 scripts/ppm_to_png.py scenes/scene3_lowres.ppm
        # one (or more) explicit paths → scenes/pngs/<stem>.png
"""
import glob
import os
import sys
import struct
import zlib

SCENES_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "scenes")
PNGS_DIR = os.path.join(SCENES_DIR, "pngs")


def read_ppm_p3(path):
    with open(path, "r") as f:
        text = f.read()
    cleaned = "\n".join(line.split("#", 1)[0] for line in text.splitlines())
    tokens = cleaned.split()

    if not tokens or tokens[0] != "P3":
        raise ValueError(f"{path}: unsupported format '{tokens[0] if tokens else ''}' (expected P3)")

    width, height, maxval = int(tokens[1]), int(tokens[2]), int(tokens[3])
    values = tokens[4 : 4 + width * height * 3]
    if len(values) != width * height * 3:
        raise ValueError(
            f"{path}: expected {width * height * 3} pixel values, found {len(values)}"
        )

    if maxval == 255:
        pixels = bytes(int(v) for v in values)
    else:
        pixels = bytes(min(255, int(v) * 255 // maxval) for v in values)
    return width, height, pixels


def write_png(path, width, height, rgb_pixels):
    def chunk(tag, data):
        return (
            struct.pack(">I", len(data))
            + tag
            + data
            + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)
        )

    stride = width * 3
    raw = bytearray()
    for y in range(height):
        raw.append(0)  # no per-scanline filter
        raw.extend(rgb_pixels[y * stride : (y + 1) * stride])

    ihdr = struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0)
    idat = zlib.compress(bytes(raw), 9)
    with open(path, "wb") as f:
        f.write(b"\x89PNG\r\n\x1a\n")
        f.write(chunk(b"IHDR", ihdr))
        f.write(chunk(b"IDAT", idat))
        f.write(chunk(b"IEND", b""))


def convert_one(ppm_path):
    width, height, pixels = read_ppm_p3(ppm_path)
    os.makedirs(PNGS_DIR, exist_ok=True)
    stem = os.path.splitext(os.path.basename(ppm_path))[0]
    png_path = os.path.join(PNGS_DIR, stem + ".png")
    write_png(png_path, width, height, pixels)
    print(f"{ppm_path} ({width}x{height}) -> {png_path}")


def main():
    if len(sys.argv) > 1:
        ppm_paths = sys.argv[1:]
    else:
        ppm_paths = sorted(glob.glob(os.path.join(SCENES_DIR, "*.ppm")))

    if not ppm_paths:
        print(f"no .ppm files found in {SCENES_DIR}")
        return

    converted = 0
    for ppm_path in ppm_paths:
        try:
            convert_one(ppm_path)
            converted += 1
        except (OSError, ValueError) as exc:
            # Incomplete leftovers (e.g. truncated out.ppm) should not abort the batch.
            print(f"skip {ppm_path}: {exc}", file=sys.stderr)

    if converted == 0:
        sys.exit(1)


if __name__ == "__main__":
    main()
