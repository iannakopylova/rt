# Ray tracer documentation

Auditor-facing guide for the Rust ray tracer in this repository (`rt`). It covers features, how to build scenes in code, how to change lights and cameras, how to write PPM images, and how to reproduce the four audit scenes.

## Features

| Feature | What you get |
|---------|----------------|
| Math | `Vec3`, `Ray`, `Color` (`src/vec3.rs`, `src/ray.rs`) |
| Camera | Pinhole `Camera::look_at` with eye, look-at, up, vertical FOV, aspect |
| Primitives | Sphere, axis-aligned cube, infinite/Y-ground plane, finite Y-cylinder |
| Scene | Object list + lights + ambient (`src/scene.rs`) |
| Shading | Lambertian diffuse, point / directional lights, shadow rays (`src/light.rs`) |
| Reflection (bonus) | `Material::metal` + `-r` / `--reflection` (`src/tracer.rs`, RT-016) |
| Refraction (bonus) | `Material::glass` + `-R` / `--refraction` (Snell's law + Fresnel, RT-017) |
| Textures (bonus) | `Material::textured` / `.with_texture(path)` + `-t` / `--textures` (P3 PPM albedo maps, `src/texture.rs`, RT-018) |
| Output | ASCII **P3 PPM** (`src/ppm.rs`) |
| CLI | `--scene`, `--width`, `--height`, `--output` (`src/main.rs`) |

Audit images live under [`scenes/`](../scenes/) (800×600). Scene builders are in [`src/scenes.rs`](../src/scenes.rs).

## Build & run

```bash
cargo build --release
cargo run --release -- --help
```

Defaults are a small preview (**400×300**, scene 1). For audit images use **800×600**.

Without `--output`, a P3 PPM is written to **stdout** (redirect with `>` if you want).

---

## Creating objects (code examples)

Imports used below:

```rust
use crate::material::Material;
use crate::objects::{Cube, Cylinder, Plane, Sphere};
use crate::scene::{Object, Scene};
use crate::vec3::{Color, Vec3};
```

### Sphere

```rust
let sphere = Sphere::with_albedo(
    Vec3::new(0.0, 0.0, -4.0), // center
    1.0,                       // radius
    Color::new(0.9, 0.25, 0.2),
);
scene.add(Object::Sphere(sphere));
```

### Cube (AABB)

```rust
let cube = Cube::with_albedo(
    Vec3::new(1.5, 0.0, -4.0), // center
    1.4,                       // edge length
    Color::new(0.25, 0.45, 0.9),
);
scene.add(Object::Cube(cube));
```

### Plane

Horizontal ground at `y = -1`:

```rust
let ground = Plane::ground(-1.0, Material::solid(Color::new(0.55, 0.55, 0.58)));
scene.add(Object::Plane(ground));
```

Arbitrary plane (point + normal):

```rust
let wall = Plane::with_albedo(
    Vec3::new(0.0, 0.0, -10.0),
    Vec3::new(0.0, 0.0, 1.0),
    Color::new(0.8, 0.8, 0.9),
);
scene.add(Object::Plane(wall));
```

### Cylinder

Finite cylinder along **+Y**, `mid` is the midpoint of the axis:

```rust
let cyl = Cylinder::with_albedo(
    Vec3::new(0.0, 0.0, -5.2), // mid-point of axis
    0.55,                      // radius
    2.0,                       // height
    Color::new(0.2, 0.75, 0.35),
);
scene.add(Object::Cylinder(cyl));
```

---

## Changing brightness (lights)

Lights live on `Scene`. Intensity is **not** distance-attenuated; brightness is controlled by `intensity`.

Constants (`src/light.rs`):

| Constant | Value | Used for |
|----------|-------|----------|
| `SCENE1_LIGHT_INTENSITY` | `1.0` | Scenes 1, 3, 4 |
| `SCENE2_LIGHT_INTENSITY` | `0.45` | Scene 2 (dimmer) |

Examples:

```rust
use crate::light::{Light, SCENE1_LIGHT_INTENSITY, SCENE2_LIGHT_INTENSITY};
use crate::vec3::{Color, Vec3};

// Scene 1 / 3 / 4 style key light
scene.add_light(Light::scene1_key(Vec3::new(3.0, 5.0, 1.0)));

// Scene 2 style (dimmer)
scene.add_light(Light::scene2_key(Vec3::new(3.0, 5.0, 1.0)));

// Custom intensity
scene.add_light(Light::point(
    Vec3::new(4.0, 6.0, 2.0),
    Color::WHITE,
    0.7, // try values between scene2 (0.45) and scene1 (1.0)
));

// Soft fill so pure shadows are not black
let scene = Scene::new().with_ambient(0.08);
```

To make Scene 2 look brighter or darker, edit `SCENE2_LIGHT_INTENSITY` in `src/light.rs`, or pass a different intensity to `Light::point`.

---

## Reflection (bonus RT-016)

Mark a material as metal, then enable recursive bounces with `-r`:

```rust
use crate::material::Material;
use crate::objects::Sphere;
use crate::vec3::{Color, Vec3};

let mirror = Sphere::new(
    Vec3::new(0.0, 0.0, -4.0),
    1.0,
    Material::metal(Color::WHITE, 0.9), // reflectivity 0..1
);
```

```bash
cargo run --release -- -s reflection -r --max-depth 5 --width 800 --height 600 -o scenes/scene_reflection.ppm
```

Without `-r`, reflective materials shade as diffuse only (fast path for audit scenes 1–4).

---

## Refraction (bonus RT-017)

Mark a material as glass (IOR, e.g. `1.5`), then enable Snell's-law refraction with `-R`. Interfaces also pick up Fresnel reflection (and total internal reflection when needed):

```rust
use crate::material::Material;
use crate::objects::Sphere;
use crate::vec3::{Color, Vec3};

let glass = Sphere::new(
    Vec3::new(0.0, 0.0, -3.5),
    1.0,
    Material::glass(Color::WHITE, 1.5), // absolute IOR
);
```

```bash
cargo run --release -- -s refraction -R --max-depth 8 --width 800 --height 600 -o scenes/scene_refraction.ppm
```

Without `-R`, dielectric materials shade as diffuse only.

---

## Textures (bonus RT-018)

Any material can carry a **P3 PPM** texture path. `texture::sample_albedo(material, uv)`
is the single point that resolves a material's shading color: it returns the
sampled texture pixel when textures are enabled and a path is set, and falls
back to `material.albedo` otherwise (no texture assigned, `-t` not passed, or
the file failed to load). Every lighting, reflection, and refraction color read
in `src/light.rs` / `src/tracer.rs` goes through this function, so reflected and
refracted rays pick up the sampled color at their **own** hit point — not the
base object's albedo and not a cached value.

### Texture file format

- **ASCII P3 PPM only** (`P3` header, `width height`, `255`, then `R G B` triples) —
  the same format this project already writes for rendered output. No image
  crate, no PNG/JPEG.
- Sampling is nearest-neighbor; out-of-range `u`/`v` wrap (tile) via `rem_euclid`.
- Demo textures live in [`textures/`](../textures/) and are regenerated with:
  ```bash
  python3 scripts/gen-demo-textures.py
  ```

### Assigning a texture

```rust
use crate::material::Material;
use crate::vec3::Color;

// Texture-only: albedo is the fallback if the file is missing or `-t` is off.
let checker = Material::textured(Color::new(0.9, 0.25, 0.2), "textures/checker_red.ppm");

// Combine with an existing material (e.g. a textured mirror).
let textured_mirror = Material::metal(Color::WHITE, 0.9).with_texture("textures/checker_blue.ppm");
```

### UV mapping per object type

| Object | Mapping | Notes |
|--------|---------|-------|
| **Sphere** | `u = 0.5 + atan2(z, x) / (2π)`, `v = 0.5 - asin(y / radius) / π` | `x,y,z` relative to center; `v = 0` at the north pole (`+Y`). |
| **Plane** | `u = (x / tile_size).rem_euclid(1)`, `v = (z / tile_size).rem_euclid(1)` | World-space X/Z at the hit point; assumes a horizontal (`+Y`) plane, matching how this project uses planes. `tile_size` defaults to `2.0`; override with `Plane::with_tile_size(n)`. |
| **Cube** | Per-face planar: drop the hit face's normal axis, normalize the other two against the cube's extent on those axes. | Faces are not seam-matched — adjacent faces tile independently. |
| **Cylinder** | `u = angle_around_axis / (2π)`, `v = height_along_axis / total_height` | Axis is world `+Y`; applies to the side wall and both caps (caps land at `v = 0` / `v = 1`). |

```rust
use crate::objects::{Cube, Cylinder, Plane, Sphere};
use crate::vec3::Vec3;

// Sphere: UV comes from the hit's own geometry — just assign a textured material.
let ball = Sphere::new(Vec3::new(0.0, 0.0, -4.0), 1.0, checker);

// Plane: optionally override the tile size (world units per repeat).
let ground = Plane::from_point_normal(
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Material::textured(Color::new(0.55, 0.55, 0.58), "textures/tile_floor.ppm"),
)
.with_tile_size(2.0);

// Cube / Cylinder: same pattern — texture_path lives on the Material.
let crate_ = Cube::from_center_extent(
    Vec3::new(1.6, 0.0, -4.0),
    1.4,
    Material::textured(Color::new(0.25, 0.45, 0.9), "textures/stripes_cube.ppm"),
);
let pillar = Cylinder::from_midpoint(
    Vec3::new(0.0, 0.0, -5.2),
    0.55,
    2.0,
    Material::textured(Color::new(0.2, 0.75, 0.35), "textures/bands_cylinder.ppm"),
);
```

### `-t` / `--textures`

```bash
cargo run --release -- -s texture-sphere -t --width 800 --height 600 -o scenes/scene_texture_sphere.ppm
cargo run --release -- -s texture-plane -t --width 800 --height 600 -o scenes/scene_texture_plane.ppm
cargo run --release -- -s 3 -t --width 800 --height 600 -o scenes/scene_texture_all.ppm
cargo run --release -- -s texture-reflection -r -t --width 800 --height 600 -o scenes/scene_texture_reflection.ppm
```

- **Without `-t`**: every material renders with its solid `albedo`, identical to
  pre-RT-018 output — `texture_path` is set on several scene objects (Scene 3's
  cube and cylinder included) but is never read. Verified by rendering
  `--scene 3` with `-t` omitted and comparing byte-for-byte (`cmp`/`md5sum`)
  against the previously-committed `scenes/scene3_all.ppm`: identical.
- **With `-t`**: materials with a `texture_path` sample their P3 PPM at the hit's
  UV; materials without one keep using solid `albedo` — no crash either way. A
  missing or invalid texture file prints one `warning:` line (memoized per
  path, not per ray hit) and falls back to `albedo`.
- Combine with `-r` / `-R`: reflection and refraction tint by the sampled
  albedo at their own hit point (see `scene_texture_reflection` above), so a
  textured mirror shows the correct texture cell in its reflection, not a flat
  color.

---

## Changing the camera (position & angle)

```rust
use crate::camera::Camera;
use crate::vec3::Vec3;

let aspect = 800.0 / 600.0;
let camera = Camera::look_at(
    Vec3::new(0.0, 2.0, 4.5), // eye (position)
    Vec3::new(0.0, 0.0, -4.2), // look-at (aim point)
    Vec3::new(0.0, 1.0, 0.0), // world up
    55.0,                      // vertical FOV in degrees
    aspect,
);
```

**Scene 4** keeps the same objects as Scene 3 (`scene3_world()`) and only swaps the camera:

| Preset | Function | Eye (approx.) |
|--------|----------|----------------|
| Scene 3 front | `scene3_camera_front` | `(0, 2, 4.5)` |
| Scene 4 alt | `scene3_camera_alt` | `(4.2, 2.8, 1.5)` |

Edit those helpers in `src/scenes.rs` to change the alternate angle.

---

## PPM output & resolution

### CLI

```bash
# Write a file
cargo run --release -- --scene 1 --width 800 --height 600 -o scenes/scene1_sphere.ppm

# Stream to stdout
cargo run --release -- --scene 1 --width 800 --height 600 > out.ppm
```

| Flag | Short | Meaning |
|------|-------|---------|
| `--scene ID` | `-s` | `1`/`sphere`, `2`/`cube`, `3`/`all`, `4`/`alt`, `5`/`reflection`, `6`/`refraction`, `7`/`texture-sphere`, `8`/`texture-plane`, `9`/`texture-reflection` |
| `--width N` | `-w` | Image width (pixels) |
| `--height N` | | Image height (pixels) |
| `--output PATH` | `-o` | Output file (omit → stdout) |
| `--reflection` | `-r` | Enable recursive reflections (RT-016) |
| `--refraction` | `-R` | Enable dielectric refraction (RT-017) |
| `--textures` | `-t` | Enable texture sampling (P3 PPM albedo maps, RT-018) |
| `--max-depth N` | | Max bounce depth for `-r` / `-R` (default 5) |
| `--help` | | Usage |

### From code

```rust
use crate::ppm::write_ppm_p3;
use crate::tracer::render_frame;
use std::fs::File;

let (scene, camera) = scenes::scene1_sphere(800.0 / 600.0);
let pixels = render_frame(&scene, &camera, 800, 600);
let mut file = File::create("out.ppm")?;
write_ppm_p3(&mut file, 800, 600, &pixels)?;
```

Format: ASCII **P3** (`P3` header, then `width height`, then `255`, then RGB triples).

---

## Reproduce all four audit images

From the repository root (PowerShell or bash):

```bash
cargo run --release -- --scene 1 --width 800 --height 600 -o scenes/scene1_sphere.ppm
cargo run --release -- --scene 2 --width 800 --height 600 -o scenes/scene2_plane_cube.ppm
cargo run --release -- --scene 3 --width 800 --height 600 -o scenes/scene3_all.ppm
cargo run --release -- --scene 4 --width 800 --height 600 -o scenes/scene4_alt_camera.ppm
```

| Scene | Ticket | CLI | Output file | Notes |
|-------|--------|-----|-------------|-------|
| 1 | RT-011 | `--scene 1` | `scenes/scene1_sphere.ppm` | Sphere + ground; light intensity `1.0` |
| 2 | RT-012 | `--scene 2` | `scenes/scene2_plane_cube.ppm` | Plane + cube; light intensity `0.45` (dimmer) |
| 3 | RT-013 | `--scene 3` | `scenes/scene3_all.ppm` | Sphere, cube, cylinder, plane; front camera |
| 4 | RT-014 | `--scene 4` | `scenes/scene4_alt_camera.ppm` | Same world as 3; alternate camera |

More detail: [`scenes/README.md`](../scenes/README.md).

### Where to tweak scenes

| What | File / symbol |
|------|----------------|
| Object layout (scenes 3–4) | `src/scenes.rs` → `scene3_world` |
| Scene 1 / 2 layouts | `scene1_sphere`, `scene2_plane_cube` |
| Cameras | `scene3_camera_front`, `scene3_camera_alt` |
| Light brightness constants | `src/light.rs` |

---

## Project layout (quick map)

```
src/
  main.rs       CLI
  scenes.rs     Audit scene builders
  scene.rs      Scene container
  tracer.rs     Closest-hit + render loop
  camera.rs     Camera
  light.rs      Lights & shading
  material.rs   Material (albedo, reflectivity, ior, texture_path)
  texture.rs    P3 PPM loader + sample_albedo (RT-018)
  objects/      Sphere, cube, plane, cylinder
  ppm.rs        P3 writer
scenes/         Generated audit PPMs + short README
textures/       Demo P3 PPM textures (RT-018)
docs/           Branching / sync notes
tickets/        Ticket board
```

## Team

See [`TEAM.md`](../TEAM.md). Ticket workflow: [`docs/BRANCHING.md`](BRANCHING.md).
