# Audit scenes

Generated P3 PPM images for the ray tracer submission.

## Brightness reference

| Scene | Helper | Intensity |
|-------|--------|-----------|
| 1 | `Light::scene1_key` | `1.0` (`SCENE1_LIGHT_INTENSITY`) |
| 2 | `Light::scene2_key` | `0.45` (`SCENE2_LIGHT_INTENSITY`) |
| 3–4 | `Light::scene1_key` | `1.0` |

## Scene 1 — sphere only (RT-011)

```powershell
cargo run --release -- --scene 1 --width 800 --height 600 -o scenes/scene1_sphere.ppm
```

## Scene 2 — plane + cube (RT-012)

```powershell
cargo run --release -- --scene 2 --width 800 --height 600 -o scenes/scene2_plane_cube.ppm
```

## Scene 3 — all four objects (RT-013)

```powershell
cargo run --release -- --scene 3 --width 800 --height 600 -o scenes/scene3_all.ppm
```

## Scene 4 — alternate camera (RT-014)

Same `scene3_world()` as Scene 3; camera from `scene3_camera_alt` (side/elevated).

```powershell
cargo run --release -- --scene 4 --width 800 --height 600 -o scenes/scene4_alt_camera.ppm
```

## Reflection demo (RT-016)

Metal sphere + red cube. Requires `-r` to see mirrors.

```powershell
cargo run --release -- -s reflection -r --width 800 --height 600 -o scenes/scene_reflection.ppm
```

Knobs: `src/scenes.rs` (`scene3_world`, `scene3_camera_front`, `scene3_camera_alt`, `scene_reflection_demo`).
