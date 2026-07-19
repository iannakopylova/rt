#!/usr/bin/env python3
"""Generate the small procedural P3 PPM textures used by the RT-018 demo scenes.

No external image assets or crates involved — these are plain ASCII PPM (P3),
the same format `src/ppm.rs` writes for rendered output. Re-run this after a
fresh clone (or if you tweak a pattern) to (re)populate `textures/`:

    python3 scripts/gen-demo-textures.py
"""
import os

OUT_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "textures")


def write_ppm(name, width, height, pixel_fn):
    path = os.path.join(OUT_DIR, name)
    with open(path, "w") as f:
        f.write("P3\n")
        f.write(f"{width} {height}\n")
        f.write("255\n")
        for y in range(height):
            row = []
            for x in range(width):
                r, g, b = pixel_fn(x, y)
                row.append(f"{r} {g} {b}")
            f.write(" ".join(row) + "\n")
    print(f"wrote {path} ({width}x{height})")


def checker(cell, c1, c2):
    def px(x, y):
        return c1 if ((x // cell) + (y // cell)) % 2 == 0 else c2

    return px


def tile_floor(x, y):
    cell, grout = 16, 2
    lx, ly = x % cell, y % cell
    if lx < grout or ly < grout:
        return (90, 90, 95)
    return (172, 172, 182)


def stripes_cube(x, y):
    cell = 8
    return (230, 140, 40) if (x // cell) % 2 == 0 else (30, 90, 90)


def bands_cylinder(x, y):
    cell = 8
    if y % cell == 0:
        return (25, 40, 25)
    band = (y // cell) % 2
    return (60, 170, 90) if band == 0 else (210, 200, 60)


def main():
    os.makedirs(OUT_DIR, exist_ok=True)
    write_ppm("checker_red.ppm", 64, 32, checker(8, (205, 45, 35), (235, 225, 200)))
    write_ppm("checker_blue.ppm", 64, 32, checker(8, (40, 95, 205), (240, 245, 250)))
    write_ppm("tile_floor.ppm", 64, 64, tile_floor)
    write_ppm("stripes_cube.ppm", 64, 64, stripes_cube)
    write_ppm("bands_cylinder.ppm", 64, 64, bands_cylinder)


if __name__ == "__main__":
    main()
