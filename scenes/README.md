# Audit scenes

Generated P3 PPM images for the ray tracer submission.

## Scene 1 — sphere only (RT-011)

One sphere above a ground plane (plane is only there so the sphere casts a visible shadow). Uses Scene 1 light brightness (`Light::scene1_key`).

```powershell
cargo run -- --scene 1 --width 800 --height 600 -o scenes/scene1_sphere.ppm
```

Dev preview (faster):

```powershell
cargo run -- --scene 1 --width 400 --height 300 -o scenes/scene1_preview.ppm
```

Sphere position / radius / camera are constants at the top of `scene1_sphere` in `src/scenes.rs`.
