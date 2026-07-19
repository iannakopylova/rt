# Audit scenes

Generated P3 PPM images for the ray tracer submission. Each `.ppm` also has a
same-named `.png` next to it (see the note at the bottom) for quick viewing —
the `.ppm` is the required deliverable; the `.png` is just a convenience copy.

## Brightness reference

| Scene | Helper | Intensity |
|-------|--------|-----------|
| 1 | `Light::scene1_key` | `1.0` (`SCENE1_LIGHT_INTENSITY`) |
| 2 | `Light::scene2_key` | `0.45` (`SCENE2_LIGHT_INTENSITY`) |
| 3–4 | `Light::scene1_key` | `1.0` |

## `scene1_sphere.ppm` / `scene1_sphere.png` (RT-011)

**What you should see:** A single sphere on a ground/background, lit by the bright key light, with a visible shadow under it.

```powershell
cargo run --release -- --scene 1 --width 800 --height 600 -o scenes/scene1_sphere.ppm
```

## `scene2_plane_cube.ppm` / `scene2_plane_cube.png` (RT-012)

**What you should see:** A flat ground plane with a cube on it, lit noticeably dimmer than Scene 1, with a visible shadow under the cube.

```powershell
cargo run --release -- --scene 2 --width 800 --height 600 -o scenes/scene2_plane_cube.ppm
```

## `scene3_all.ppm` / `scene3_all.png` (RT-013)

**What you should see:** All four object types in one scene — sphere, cube, cylinder, flat plane — all solid colors (no textures), each casting a visible shadow.

```powershell
cargo run --release -- --scene 3 --width 800 --height 600 -o scenes/scene3_all.ppm
```

## `scene4_alt_camera.ppm` / `scene4_alt_camera.png` (RT-014)

**What you should see:** The exact same scene as `scene3_all`, viewed from a different camera position/angle.

```powershell
cargo run --release -- --scene 4 --width 800 --height 600 -o scenes/scene4_alt_camera.ppm
```

## `scene_reflection.ppm` / `scene_reflection.png` (RT-016 bonus)

**What you should see:** A mirror-like metal sphere next to a red cube — the cube (and/or surroundings) should be visibly reflected in the sphere's surface.

```powershell
cargo run --release -- -s reflection -r --width 800 --height 600 -o scenes/scene_reflection.ppm
```

## `scene_refraction.ppm` / `scene_refraction.png` (RT-017 bonus)

**What you should see:** A glass-like sphere with a red cube behind it — the cube should look bent/distorted when seen through the sphere, like looking through actual glass.

```powershell
cargo run --release -- -s refraction -R --width 800 --height 600 -o scenes/scene_refraction.ppm
```

## `scene_texture_sphere.ppm` / `scene_texture_sphere.png` (RT-018 bonus)

**What you should see:** A single sphere with a checkerboard pattern wrapped around its surface (not a flat solid color), over a plain ground.

```powershell
cargo run --release -- -s texture-sphere -t --width 800 --height 600 -o scenes/scene_texture_sphere.ppm
```

## `scene_texture_plane.ppm` / `scene_texture_plane.png` (RT-018 bonus)

**What you should see:** A ground plane with a repeating tiled pattern on it, with a plain solid-color sphere sitting on top for scale/shadow.

```powershell
cargo run --release -- -s texture-plane -t --width 800 --height 600 -o scenes/scene_texture_plane.ppm
```

## `scene_texture_all.ppm` / `scene_texture_all.png` (RT-018 bonus)

**What you should see:** All four objects together — sphere and ground plane are plain solid colors, the cube shows vertical stripes, the cylinder shows horizontal bands. This mix is intentional — it proves textured and untextured materials work in the same scene.

```powershell
cargo run --release -- -s 3 -t --width 800 --height 600 -o scenes/scene_texture_all.ppm
```

## `scene_texture_reflection.ppm` / `scene_texture_reflection.png` (RT-018 + RT-016 combined)

**What you should see:** A mirror sphere that also has the checkerboard texture on it — the checker pattern should be visible both directly on the sphere's surface AND faintly inside its own reflection.

```powershell
cargo run --release -- -s texture-reflection -r -t --width 800 --height 600 -o scenes/scene_texture_reflection.ppm
```

Knobs: `src/scenes.rs` (`scene3_world`, `scene3_camera_front`, `scene3_camera_alt`, `scene_reflection_demo`, `scene_refraction_demo`, `scene_texture_sphere_demo`, `scene_texture_plane_demo`, `scene_texture_reflection_demo`).

---

Run `python3 scripts/ppm_to_png.py` any time a `.png` preview above is missing or stale (e.g. after re-rendering a `.ppm`, or after a fresh clone if the PNGs weren't committed) — it regenerates every `scenes/*.png` from its matching `.ppm`. Note on `.gitignore`: `*.ppm` is ignored repo-wide except under `scenes/` and `textures/` (both explicitly un-ignored); `.png` has no matching rule at all, so `scenes/*.png` files aren't excluded from version control the way a bare `.ppm` elsewhere would be — they're ordinary trackable files, and only need regenerating if they're actually missing from your working copy.
