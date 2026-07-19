# rt — Audit checklist (with answers & how to check)

Use this while grading or self-checking the Rust ray tracer.

## Before you start

```bash
cd rt
cargo build --release
cargo test          # should report all tests ok (106+ as of this write-up)
cargo run --release -- --help
```

**Where things live**

| Path | What |
|------|------|
| `scenes/*.ppm` | Required audit / bonus deliverable images (P3 PPM) |
| `scenes/pngs/*.png` | Convenience previews (optional to open in a viewer) |
| `docs/DOCUMENTATION.md` | How to create objects, change brightness, move the camera |
| `scenes/README.md` | Exact regenerate commands for every image |

**Defaults:** CLI preview is **400×300**, scene 1. For audit-quality images use **`--width 800 --height 600`**.

**Viewing PPMs:** open in an image viewer that supports P3 PPM, or convert:

```bash
python3 scripts/ppm_to_png.py   # writes scenes/pngs/<name>.png from each scenes/*.ppm
```

Then open files under `scenes/pngs/`.

**Important — `-o` is the output path.** `--width` / `--height` only change the file you write with `-o`. They do **not** edit an existing PPM/PNG. Example: `-o /tmp/preview.ppm` never updates `scenes/scene3_all.ppm` or `scenes/pngs/scene3_all.png`.

---

#### Functional

##### Using the ray tracer construct any scene you want, including at least one of all objects. (this can take a while to render, so in the meantime you can skip the first two questions and answer the following ones)

**How to execute**

```bash
# Scene 3 already contains sphere + cube + cylinder + plane (audit size)
cargo run --release -- --scene 3 --width 800 --height 600 -o scenes/scene3_all.ppm
python3 scripts/ppm_to_png.py scenes/scene3_all.ppm
# → open scenes/pngs/scene3_all.png
```

Fast preview while iterating (separate file — does not replace the audit PPM):

```bash
cargo run --release -- --scene 3 --width 200 --height 150 -o scenes/scene3_preview.ppm
python3 scripts/ppm_to_png.py scenes/scene3_preview.ppm
# → open scenes/pngs/scene3_preview.png
```

###### Does the image correspond to the scene you created?

**Answer: Yes.**  
**How:** Open `scenes/scene3_all.ppm` or `scenes/pngs/scene3_all.png`. You should see a ground plane plus a sphere, a cube, and a cylinder under a key light, with shadows. Layout is built in `src/scenes.rs` → `scene3_world()`.

###### Is it possible for you to reduce the resolution of the output image?

**Answer: Yes.**  
**How:** Pass smaller `--width` / `--height` (or `-w` / `-h`) and write a **new** file. Then check the PPM header (line 2 is `width height`):

```bash
# Low-res proof file (keeps the 800×600 audit PPM untouched)
cargo run --release -- --scene 3 --width 320 --height 240 -o scenes/scene3_lowres.ppm

# 1) Confirm resolution in the PPM itself
head -n 3 scenes/scene3_lowres.ppm
# expect:
#   P3
#   320 240
#   255

# Compare with the audit render (still 800×600):
 
# expect: 800 600

# 2) Convert the low-res PPM so you can view it as PNG
python3 scripts/ppm_to_png.py scenes/scene3_lowres.ppm
# → open scenes/pngs/scene3_lowres.png  (visibly smaller / blockier than scene3_all.png)
```

##### Move the camera and render the same scene.

**How to execute**

```bash
# Same objects as scene 3, different camera
cargo run --release -- --scene 4 --width 800 --height 600 -o scenes/scene4_alt_camera.ppm
```

Camera helpers: `scene3_camera_front` vs `scene3_camera_alt` in `src/scenes.rs`.

###### Does the image correspond to the same scene, but from a different perspective?

**Answer: Yes.**  
**How:** Compare `scene3_all.ppm` and `scene4_alt_camera.ppm`. Same objects/lights; viewpoint differs (front vs angled). Unit test `scene4_reuses_scene3_world` / `scene4_camera_differs_from_scene3` also encodes this.

###### Did the student provide 4 .ppm pictures?

**Answer: Yes.**  
**How:** Check these four exist under `scenes/`:

1. `scene1_sphere.ppm`
2. `scene2_plane_cube.ppm`
3. `scene3_all.ppm`
4. `scene4_alt_camera.ppm`

Regenerate if missing:

```bash
cargo run --release -- --scene 1 --width 800 --height 600 -o scenes/scene1_sphere.ppm
cargo run --release -- --scene 2 --width 800 --height 600 -o scenes/scene2_plane_cube.ppm
cargo run --release -- --scene 3 --width 800 --height 600 -o scenes/scene3_all.ppm
cargo run --release -- --scene 4 --width 800 --height 600 -o scenes/scene4_alt_camera.ppm
```

###### Does one of these images consist of a scene with a sphere?

**Answer: Yes — `scene1_sphere.ppm`.**  
**How:** Open it; a single sphere (plus ground) is the main subject. Command: `--scene 1`.

###### Does one of these images consist of a scene with a flat plane and a cube with lower brightness than in the sphere image?

**Answer: Yes — `scene2_plane_cube.ppm`.**  
**How:** Plane + cube; Scene 2 key light intensity is **0.45** vs Scene 1’s **1.0** (`Light::scene2_key` / `SCENE2_LIGHT_INTENSITY`). Image should look clearly dimmer than Scene 1. Command: `--scene 2`.

###### Does one of these images consist of a scene with one of each of all the objects (one cube, one sphere, one cylinder and one flat plane)?

**Answer: Yes — `scene3_all.ppm`.**  
**How:** Identify all four shapes. Command: `--scene 3`. Covered by `scenes::tests::scene3_has_all_four_primitives`.

###### Does one of these images consist of a scene like the previous one, but with the camera in another position (thus generating the same image from a different perspective)?

**Answer: Yes — `scene4_alt_camera.ppm`.**  
**How:** Same world as Scene 3; camera moved. Command: `--scene 4`.

###### Considering all of the previous pictures, can you see shadows from the objects?

**Answer: Yes.**  
**How:** Look under spheres/cubes/cylinders on the ground plane — darker contact/shadow regions from shadow rays (`src/light.rs` / tracer). Scene 1 and 2 make this especially clear.

###### Did the student provide clear documentation for the ray tracer on how to use it (create elements, change brightness and move the camera)?

**Answer: Yes.**  
**How:** Read in order:

1. [`README.md`](../README.md) — build / quick start / feature list  
2. [`docs/DOCUMENTATION.md`](DOCUMENTATION.md) — **auditor usage guide**: add sphere/cube/plane/cylinder, lights & brightness, camera `look_at`, CLI flags, how to reproduce all audit images  
3. [`DOCUMENTATION.md`](../DOCUMENTATION.md) — architecture / math deep dive  
4. [`scenes/README.md`](../scenes/README.md) — per-image regenerate commands  

Quick spot-checks in `docs/DOCUMENTATION.md`: sections “Creating elements”, “Changing brightness (lights)”, “Changing the camera”.

---

#### Unit Tests

###### Do all tests pass without errors?

**Answer: Yes.**  
**How to execute**

```bash
cargo test
```

Expect a summary like `test result: ok. 106 passed; 0 failed; ...` (count may grow; zero failures is what matters).

###### Are there specific tests for **Ray-Object Intersections** (e.g., verifying that a ray correctly detects hitting a sphere or a cylinder)?

**Answer: Yes.**  
**How:** Tests live next to the objects and scenes, for example:

- `src/objects/sphere.rs`, `cube.rs`, `cylinder.rs`, `plane.rs` — `#[cfg(test)]` intersection cases  
- `scenes::tests::scene3_rays_hit_each_object_type`  
- Tracer hit/miss tests in `src/tracer.rs`

Filter example:

```bash
cargo test intersect
cargo test sphere
cargo test cylinder
```

###### Are there tests for **Vector Math** (e.g., ensuring dot products and normalization are calculated correctly)?

**Answer: Yes.**  
**How:** `src/vec3.rs` tests — `dot_and_cross`, `length_and_normalize`, `add_and_sub`, `scalar_mul`, etc.

```bash
cargo test vec3
```

###### Are there tests for **Light and Color** logic, such as ensuring brightness changes don't cause overflow in the RGB values?

**Answer: Yes.**  
**How:**

- `vec3::tests::color_clamp_and_rgb8` — RGB clamping / byte conversion  
- `light.rs` tests — lighting helpers  
- `tracer::tests::scene2_brightness_darker_than_scene1` — Scene 2 darker than Scene 1  
- Shading tests that hits are not flat albedo (`hit_is_shaded_not_flat_albedo`)

```bash
cargo test color
cargo test brightness
cargo test light
```

---

#### Bonus

###### +Is it possible to add textures to the surface of the objects?

**Answer: Yes.**  
**How to execute / check**

```bash
cargo run --release -- -s texture-sphere -t --width 800 --height 600 -o scenes/scene_texture_sphere.ppm
cargo run --release -- -s texture-plane -t --width 800 --height 600 -o scenes/scene_texture_plane.ppm
cargo run --release -- -s 3 -t --width 800 --height 600 -o scenes/scene_texture_all.ppm
```

Open the PPMs/PNGs: checker / tiles / stripes / bands instead of flat solid colors. Docs: `docs/DOCUMENTATION.md` (textures) and `scenes/README.md`. Flag: `-t` / `--textures`.

###### +Is it possible to make reflective and/or refractive objects?

**Answer: Yes (both).**  
**How to execute / check**

```bash
# Reflection (mirror sphere)
cargo run --release -- -s reflection -r --width 800 --height 600 -o scenes/scene_reflection.ppm

# Refraction (glass sphere)
cargo run --release -- -s refraction -R --width 800 --height 600 -o scenes/scene_refraction.ppm
```

- **Reflection:** cube (or surroundings) visible in the metal sphere (`-r` / `--reflection`).  
- **Refraction:** backdrop looks bent through the glass sphere (`-R` / `--refraction`).  
Optional: `-r -t` together → `scene_texture_reflection.ppm`.

###### +Is it possible to add particles?

**Answer: No.**  
**How:** Not implemented — no particle system / emitter API. Do not claim this bonus.

###### +Is it possible to add fluids?

**Answer: No.**  
**How:** Not implemented — no fluid / volume simulation. Do not claim this bonus.

---

## Auditor walkthrough (recommended order)

1. **`cargo test`** — all green.  
2. Confirm **four audit PPMs** under `scenes/` (or regenerate with the commands above).  
3. Open Scene 1–4 (PPM or PNG) and tick sphere / dimmer plane+cube / all four objects / alt camera / shadows.  
4. Change resolution once (`--width 200 --height 150`) to prove control.  
5. Skim **`docs/DOCUMENTATION.md`** for objects, brightness, camera.  
6. **Bonus (optional):** run texture / reflection / refraction commands; skip particles & fluids.

## One-liner regenerate (all core audit scenes)

```bash
cargo run --release -- --scene 1 --width 800 --height 600 -o scenes/scene1_sphere.ppm && \
cargo run --release -- --scene 2 --width 800 --height 600 -o scenes/scene2_plane_cube.ppm && \
cargo run --release -- --scene 3 --width 800 --height 600 -o scenes/scene3_all.ppm && \
cargo run --release -- --scene 4 --width 800 --height 600 -o scenes/scene4_alt_camera.ppm
```
