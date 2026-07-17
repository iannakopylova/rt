# Audit scenes

Generated P3 PPM images for the ray tracer submission.

## Brightness reference

| Scene | Helper | Intensity |
|-------|--------|-----------|
| 1 | `Light::scene1_key` | `1.0` (`SCENE1_LIGHT_INTENSITY`) |
| 2 | `Light::scene2_key` | `0.45` (`SCENE2_LIGHT_INTENSITY`) |

## Scene 1 — sphere only (RT-011)

One sphere above a ground plane (plane is only there so the sphere casts a visible shadow). Uses Scene 1 light brightness.

```powershell
cargo run --release -- --scene 1 --width 800 --height 600 -o scenes/scene1_sphere.ppm
```

## Scene 2 — plane + cube (RT-012)

One plane + one cube. Same light position as Scene 1, but **lower** brightness (`0.45`).

```powershell
cargo run --release -- --scene 2 --width 800 --height 600 -o scenes/scene2_plane_cube.ppm
```

Dev preview (faster):

```powershell
cargo run -- --scene 2 --width 400 --height 300 -o scenes/scene2_preview.ppm
```

Object / camera knobs live in `src/scenes.rs` (`scene1_sphere`, `scene2_plane_cube`).
